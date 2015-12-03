use config;

use fs2::FileExt;

use std::env;
use std::fs::{File, metadata};
use std::io;
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
    try!(file.try_lock_exclusive());
    println!("in lock, file was locked");
    Ok(file)
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
