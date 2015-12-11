extern crate atomicwrites;
extern crate libc;
extern crate openssl;
extern crate protobuf;
extern crate rust_multihash;
extern crate rustc_serialize;

mod ipfs_commands;
mod block;
mod blockstore;
mod commands;
mod config;
mod crypto;
mod fsrepo;
mod merkledag;
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
        let (cmd, args, opts) = try!(commands::cli::parse(args, root));
        let req = request::Request::new(cmd, args, opts, context);
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
        Err(e) => { println!("{}", e); return },
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
                           vec![
                            ("init", ipfs_commands::init::make_command()),
                            ("add", ipfs_commands::add::make_command()),
                            ("file", ipfs_commands::file::make_command()),
                           ])
}
