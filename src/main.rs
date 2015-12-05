extern crate atomicwrites;
extern crate libc;
extern crate openssl;
extern crate rust_multihash;
extern crate rustc_serialize;

mod block;
mod blockstore;
mod commands;
mod config;
mod crypto;
mod init;
mod fsrepo;
mod root; // TODO: where should this module reside?
mod util;

use commands::request;

use std::env;
use std::fs::{self, File};
use std::io;
use std::path;

struct CommandInvocation<'a> {
    pub request: request::Request<'a>,
    pub command: &'a commands::Command,
}

type ParseError = String;

impl<'a> CommandInvocation<'a> {
    fn from_parse<I>(args: I, root: &'a commands::Command, context: request::Context)
                -> Result<CommandInvocation<'a>, ParseError>
        where I : Iterator<Item=String>
    {
        let (opts, cmd) = try!(commands::cli::parse(args, root));
        let req = request::Request::new(opts, cmd, context);
        Ok(CommandInvocation { request: req, command: cmd })
    }

    fn run(&self) -> Result<(), String> {
        self.command.run(&self.request)
    }
}

fn main() {
    let root = make_root_command();

    let context = match fsrepo::best_known_path() {
        Err(e) => panic!("{}", e),
        Ok(path) => request::Context::new(path),
    };

    let invoc = match CommandInvocation::from_parse(env::args().skip(1), &root, context) {
        Err(e) => panic!("{}", e),
        Ok(invoc) => invoc,
    };

    match invoc.run() {
        Err(e) => println!("{}", e),
        _ => {},
    }
}

fn make_root_command() -> commands::Command {
    let short_help = commands::Opt::new_bool(
        vec!["h"],
        "Show a short version of the command help text"
    );

    let long_help = commands::Opt::new_bool(
        vec!["help"],
        "Show the full command help text"
    );

    fn run(req: &request::Request)  -> Result<(), String> {
        println!("{}\n{}\n{}",
                 req.command.help_text.tagline,
                 req.command.help_text.short_desc,
                 req.command.help_text.synopsis);
        Ok(())
    }

    commands::Command::new(vec![short_help, long_help],
                           vec![],
                           run,
                           root::RootHelpText,
                           vec![("init", make_init_command())])
}

fn make_init_command() -> commands::Command {
    let force = commands::Opt::new_bool(
        vec!["f", "force"],
        "Overwrite existing configuration (if it exists)"
    );

    fn run(req: &request::Request)  -> Result<(), String> {
        let repo_dir = req.context.repo_dir.clone();
        if try!(fsrepo::is_locked(repo_dir.clone())) {
            return Err("Another process has locked the repo. Unable to continue.".to_string());
        }

        try!(check_and_prepare_repo_dir(repo_dir.clone()));

        if fsrepo::is_initialized(repo_dir.clone()) {
            if req.option("f").is_some() {
                try!(fsrepo::remove(&repo_dir));
            } else {
                return Err("IPFS repo already exists.\n\
                            Reinitializing would overwrite your keys.\n\
                            (Use -f to force reinitialization.)".to_string())
            }
        }

        let config = config::init(config::DEFAULT_KEYPAIR_NUM_BITS);

        fsrepo::init(repo_dir, &config)
    }

    commands::Command::new(vec![force], vec![], run, init::InitHelpText, vec![])
}


// if the directory exists, try creating a file in it.
// if the directory doesnt exist, try to create it
// if either of these fail, return an error
fn check_and_prepare_repo_dir(mut repo_path: path::PathBuf) -> Result<(), String> {
    match fs::metadata(&repo_path) {
        Err(e) =>
            match e.kind() {
                io::ErrorKind::NotFound =>
                    fs::create_dir_all(repo_path)
                        .map_err(|e| format!("Error creating repo directory: {}", e)),
                _ => Err(format!("Error checking repo directory: {}", e)),
            },
        Ok(_) => {
            repo_path.push("test");
            // discard the File as we don't need it.
            try!(File::create(&repo_path)
                    .map_err(|e| format!("Error creating test file: {}", e)));
            fs::remove_file(&repo_path)
                .map_err(|e| format!("Error removing test file: {}", e))
        },
    }
}
