use config;

use std::env;
use std::path::PathBuf;

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
