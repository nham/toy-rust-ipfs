extern crate openssl;
extern crate rust_multihash;
#[cfg(test)] extern crate rustc_serialize;

mod block;
mod blockstore;
mod commands;
mod init;
mod root; // TODO: where should this module reside?
mod util;

use std::env;

struct CommandInvocation<'a> {
    pub request: commands::request::Request<'a>,
    pub command: &'a commands::Command,
}

type ParseError = String;

impl<'a> CommandInvocation<'a> {
    fn parse<I>(args: I, root: &'a commands::Command)
                -> Result<CommandInvocation<'a>, ParseError>
        where I : Iterator<Item=String>
    {
        let (req, cmd) = try!(commands::cli::parse(args, root));
        Ok(CommandInvocation { request: req, command: cmd })
    }

    fn run(&self) -> Result<(), String> {
        self.command.run(&self.request)
    }
}

fn main() {
    let root = make_root_command();

    let invoc = match CommandInvocation::parse(env::args().skip(1), &root) {
        Err(e) => panic!("{}", e),
        Ok(invoc) => invoc,
    };

    for (k, v) in invoc.request.options() {
        println!("{}: {:?}", k, v);
    }

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

    fn run(req: &commands::request::Request)  -> Result<(), String> {
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
    fn run(req: &commands::request::Request)  -> Result<(), String> {
        println!("Hello from the init command!");
        Ok(())
    }

    commands::Command::new(vec![], vec![], run, init::InitHelpText, vec![])
}
