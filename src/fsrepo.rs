use config;

use libc;

use std::env;
use std::fs::{File, metadata};
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

const LOCK_FILE: &'static str = "repo.lock";

pub fn is_locked(mut repo_path: PathBuf) -> Result<bool, String> {
    repo_path.push(LOCK_FILE);

    println!("in is_locked, testing for file {:?}", &repo_path);
    match metadata(repo_path.clone()) {
        Err(e) =>
            if let io::ErrorKind::NotFound = e.kind() {
                return Ok(false);
            },
        _ => {},
    }

    println!("in is_locked, file exists");
    match lock(repo_path) {
        Err(e) =>
            match e.kind() {
                io::ErrorKind::WouldBlock => Ok(true),
                _ => Err(format!("{}", e)),
            },
        Ok(_) => Ok(false),
    }
}

pub fn lock(mut repo_path: PathBuf) -> Result<File, io::Error> {

    let file = try!(File::create(repo_path));
    println!("in lock, file created");
    try!(fcntl(&file, 6)); // no F_SETLK in libc. :(
    println!("in lock, file was locked");
    Ok(file)
}

fn fcntl(file: &File, cmd: libc::c_int) -> Result<(), io::Error> {
    // this only works on 64-bit linux probably
    #[derive(Debug)]
    struct flock64 {
        ty: libc::c_int,
        whence: libc::c_int,
        start: libc::off_t,
        len: libc::off_t,
        pid: libc::pid_t,
    }

    let flock = flock64 {
        ty: 0x1, // there's no F_RDLCK / F_WRLCK / F_UNLCK in libc :(
                 // this is F_WRLCK, btw, I think
        whence: libc::SEEK_SET,
        start: 0,
        len: 0,
        pid: unsafe { libc::getpid() },
    };

    println!("{:?}", flock);

    let ret = unsafe { libc::fcntl(file.as_raw_fd(), cmd, &flock) };
    if ret < 0 { Err(io::Error::last_os_error()) } else { Ok(()) }
}

// Use the environment variable to determine repo path if it exists,
// else use the default path.
pub fn best_known_path() -> Result<PathBuf, String> {
    // TODO: this should probably use var_os, but having to handle OsString
    // complicates things
    let unexpanded_path = match env::var(config::ENV_NAME_REPO_DIR) {
        Err(_) => {
            config::DEFAULT_REPO_ROOT.to_string()
            + config::DEFAULT_REPO_PATH
        }
        Ok(s) => s,
    };

    expand_tilde(unexpanded_path).map_err(|e| format!("Could not expand repo path"))
}

fn expand_tilde(s: String) -> Result<PathBuf, ()> {
    if s.len() > 0 && s.starts_with("~") {
        match env::home_dir() {
            None => Err(()),
            Some(mut dir) => {
                dir.push(&s[1..]);
                Ok(PathBuf::from(dir))
            }
        }
    } else {
        Ok(PathBuf::from(s))
    }
}
