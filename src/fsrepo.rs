use config;
use util;

use libc;

use atomicwrites::{AtomicFile, DisallowOverwrite};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

const LOCK_FILE: &'static str = "repo.lock";
const DATASTORE_DIR: &'static str = "datastore";
const BLOCKSTORE_DIR: &'static str = "blocks";
const LOGS_DIR: &'static str = "logs";

// TODO: make this work across multiple threads?
pub fn is_locked(mut repo_path: PathBuf) -> Result<bool, String> {
    repo_path.push(LOCK_FILE);

    match util::file_exists(&repo_path) {
        Ok(false) => return Ok(false),
        _ => {},
    }

    match lock(&repo_path) {
        Err(e) =>
            match e.kind() {
                // TODO: is WouldBlock correct here?
                io::ErrorKind::WouldBlock => Ok(true),
                _ => Err(format!("{}", e)),
            },
        Ok(_) => Ok(false),
    }
}

// TODO: make this work across multiple threads?
pub fn lock(repo_path: &PathBuf) -> Result<File, io::Error> {
    let file = try!(File::create(repo_path));
    try!(fcntl(&file, 6)); // no F_SETLK in libc. :(
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
                dir.push(&s[2..]);
                Ok(PathBuf::from(dir))
            }
        }
    } else {
        Ok(PathBuf::from(s))
    }
}


// TODO: ensure this process can't modify the repo while this check is in progress
// assumes that we have sufficient permission to the repo directory,
// so doesn't worry about any permissions errors from checking existence
pub fn is_initialized(mut repo_path: PathBuf) -> bool  {
    let config_path = config::repo_path_to_config_file(repo_path.clone());
    // unwrap is "safe" here because the function assumes that there are no
    // permissions errors, so file_exists() should not return an error
    if !util::file_exists_expect(config_path) {
        return false;
    }

    // TODO: why does the analogous function in go-ipfs only check the
    // datastore directory? what about blocks and log directories
    repo_path.push(DATASTORE_DIR);
    if !util::file_exists_expect(repo_path) {
        return false;
    }

    true
}

pub fn remove<P: AsRef<Path>>(repo_path: P) -> Result<(), String> {
    fs::remove_dir_all(repo_path).map_err(|e| format!("Error removing repo: {}", e))
}

pub fn init(mut repo_path: PathBuf, cfg: &config::Config) -> Result<(), String> {
    // Don't initialize if already initialized.
    if is_initialized(repo_path.clone()) {
        return Ok(());
    }

    let config_path = config::repo_path_to_config_file(repo_path.clone());
    try!(write_config_file(config_path, cfg));

    let mut datastore_path = repo_path.clone();
    datastore_path.push(DATASTORE_DIR);
    try!(util::ensure_dir_writable(datastore_path)
            .map_err(|e| format!("Error checking writability of datastore dir: {}", e)));

    let mut blockstore_path = repo_path.clone();
    blockstore_path.push(BLOCKSTORE_DIR);
    try!(util::ensure_dir_writable(blockstore_path)
            .map_err(|e| format!("Error checking writability of blockstore dir: {}", e)));

    let mut logs_path = repo_path.clone();
    logs_path.push(LOGS_DIR);
    try!(util::ensure_dir_writable(logs_path)
            .map_err(|e| format!("Error checking writability of logs dir: {}", e)));

    Ok(())
}

// Check that the config file doesn't exist before calling this
fn init_config(mut repo_path: PathBuf) {
    let config_path = config::repo_path_to_config_file(repo_path.clone());
}

// Caller should ensure the directory exists before calling
fn write_config_file<P: AsRef<Path>>(file_path: P, cfg: &config::Config) -> Result<(), String> {
    let s = match cfg.to_json_string() {
        Err(e) => return Err(format!("Error encoding config as Json: {}", e)),
        Ok(s) => s,
    };

    let file = AtomicFile::new(file_path, DisallowOverwrite);
    file.write(|f| f.write_all(s.as_bytes()))
        .map_err(|e| format!("Error writing config file: {}", e))
}
