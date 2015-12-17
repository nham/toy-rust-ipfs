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
mod core;
mod crypto;
mod fsrepo;
mod merkledag;
mod root; // TODO: where should this module reside?
mod util;
mod unixfs;

use blockstore::Blockstore;
use commands::request;
use core::IpfsNode;

use std::env;
use std::path::PathBuf;

struct CommandInvocation<'a, 'b> {
    pub request: request::Request<'a, 'b>,
    pub command: &'a commands::Command,
}

type ParseError = String;

impl<'a, 'b> CommandInvocation<'a, 'b> {
    fn from_cli_parse<I>(args: I,
                     root: &'a commands::Command,
                     context: request::Context<'b>)
                     -> Result<Self, ParseError>
        where I: Iterator<Item = String>
    {
        let (cmd, args, opts) = try!(commands::cli::parse(args, root));
        let req = request::Request::new(cmd, args, opts, context);
        Ok(CommandInvocation {
            request: req,
            command: cmd,
        })
    }

    fn run(&mut self) -> Result<(), String> {
        self.command.run(&mut self.request)
    }
}

fn main() {
    let root = make_root_command();

    let path = match fsrepo::best_known_path() {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(path) => path,
    };

    let node = match construct_node(path.clone()) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(node) => node,
    };

    let context = request::Context::new(path, node.as_ref());

    let mut invoc = match CommandInvocation::from_cli_parse(env::args().skip(1),
                                                        &root,
                                                        context) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(invoc) => invoc,
    };

    match invoc.run() {
        Err(e) => println!("{}", e),
        _ => {}
    }
}

fn construct_node(repo_path: PathBuf) -> Result<Option<IpfsNode>, String> {
    if !try!(fsrepo::is_initialized(repo_path.clone())) {
        return Ok(None);
    }
    let config_path = config::repo_path_to_config_file(repo_path.clone());
    let config = try!(fsrepo::read_config_file(&config_path));
    let mut blockstore_path = repo_path;
    blockstore_path.push(blockstore::BLOCKSTORE_DIR);
    let bs = Blockstore::new(blockstore_path);
    Ok(Some(IpfsNode::new(bs, config)))
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

    fn run(req: &request::Request) -> Result<(), String> {
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
                            ("object", ipfs_commands::object::make_command()),
                           ])
}
