extern crate openssl;
extern crate rust_multihash;
#[cfg(test)] extern crate rustc_serialize;

mod block;
mod blockstore;
mod commands;
mod root; // TODO: where should this module reside?
mod util;

use std::env;

struct CommandInvocation<'a> {
    request: commands::request::Request,
    command: &'a commands::Command,
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
}

fn main() {
    println!("Hello, universe.");

    let root = make_root_command();

    let invoc = CommandInvocation::parse(env::args().skip(1), &root);
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


    commands::Command::new(vec![short_help, long_help],
                           vec![],
                           root::RootHelpText,
                           vec![])
}
