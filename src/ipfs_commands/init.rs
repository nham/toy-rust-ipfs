use commands::{self, HelpText, Command};
use commands::request;
use config;
use fsrepo;
use util;

use std::fs::{self, File};
use std::io;
use std::path;

const InitHelpText: HelpText = HelpText {
    tagline: "Initializes IPFS config file",
    synopsis: "",
    short_desc: "Initializes IPFS configuration files and generates a new keypair.",
};

fn run(req: &request::Request) -> Result<(), String> {
    let repo_dir = req.context.repo_dir.clone();
    if try!(fsrepo::is_locked(repo_dir.clone())) {
        return Err("Another process has locked the repo. Unable to continue."
                       .to_string());
    }

    try!(check_and_prepare_repo_dir(repo_dir.clone()));

    if try!(fsrepo::is_initialized(repo_dir.clone())) {
        if req.option("f").is_some() {
            try!(fsrepo::remove(&repo_dir));
            try!(util::ensure_dir_writable(&repo_dir).map_err(|e| {
                format!("Error ensuring repo directory is writable after forced \
                         removal: {}",
                        e)
            }));
        } else {
            return Err("IPFS repo already exists.\nReinitializing would overwrite your \
                        keys.\n(Use -f to force reinitialization.)"
                           .to_string());
        }
    }

    let config = config::init(config::DEFAULT_KEYPAIR_NUM_BITS);

    fsrepo::init(repo_dir, &config)
}

ipfs_command!(InitCommand, run);

pub fn make_command() -> Box<Command> {
    let force = commands::Opt::new_bool(
        vec!["f", "force"],
        "Overwrite existing configuration (if it exists)"
    );


    Box::new(InitCommand::new("init", vec![force], vec![], InitHelpText, vec![]))
}


// if the directory exists, try creating a file in it.
// if the directory doesnt exist, try to create it
// if either of these fail, return an error
fn check_and_prepare_repo_dir(mut repo_path: path::PathBuf) -> Result<(), String> {
    match fs::metadata(&repo_path) {
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    fs::create_dir_all(repo_path).map_err(|e| {
                        format!("Error creating repo directory: {}", e)
                    })
                }
                _ => Err(format!("Error checking repo directory: {}", e)),
            }
        }
        Ok(_) => {
            repo_path.push("test");
            // discard the File as we don't need it.
            try!(File::create(&repo_path)
                     .map_err(|e| format!("Error creating test file: {}", e)));
            fs::remove_file(&repo_path)
                .map_err(|e| format!("Error removing test file: {}", e))
        }
    }
}
