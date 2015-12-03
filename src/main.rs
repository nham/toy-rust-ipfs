extern crate fs2;
extern crate openssl;
extern crate rust_multihash;
#[cfg(test)] extern crate rustc_serialize;

mod block;
mod blockstore;
mod commands;
mod config;
mod init;
mod fsrepo;
mod root; // TODO: where should this module reside?
mod util;

use commands::request;

use std::env;

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

    invoc.run();
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
    fn run(req: &request::Request)  -> Result<(), String> {
        if try!(fsrepo::is_locked(req.context.repo_dir.clone())) {
            return Err("Another process has locked the repo. Unable to continue.".to_string());
        }
        println!("Hello from the init command!");
        Ok(())
    }

    commands::Command::new(vec![], vec![], run, init::InitHelpText, vec![])
}
