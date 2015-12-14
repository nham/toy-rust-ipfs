use super::Command;
use blockstore::{self, Blockstore};
use config;
use core::IpfsNode;
use util;

use std::collections::HashMap;
use std::collections::hash_map;
use std::path::PathBuf;

// An option submitted for a request.
#[derive(Debug)]
pub enum Opt {
    String(String),
    Bool(bool),
    Int(i32),
}

impl Opt {
    pub fn parse_string(s: String, opt_type: super::OptType) -> Result<Self, String> {
        match opt_type {
            super::OptType::String => Ok(Opt::String(s)),
            super::OptType::Bool => {
                if s == "true" {
                    Ok(Opt::Bool(true))
                } else if s == "false" {
                    Ok(Opt::Bool(false))
                } else {
                    Err("Expected boolean value for boolean option.".to_string())
                }
            }
            super::OptType::Int => {
                let x = try!(s.parse::<i32>()
                              .map_err(|e| format!("{}", e)));
                Ok(Opt::Int(x))
            }
        }
    }
}

#[derive(Debug)]
pub struct FileArg {
    path: PathBuf
}

impl FileArg {
    pub fn new(s: String) -> Result<Self, String> {
        let path = PathBuf::from(s);
        match util::file_exists(&path) {
            Ok(true) => {},
            Ok(false) => return Err(format!("File {:?} does not exist", path)),
            Err(e) => return Err(format!("Error checking existence of file {:?}: {}", path, e)),
        }

        Ok(FileArg { path: path })
    }
}

#[derive(Debug)]
pub enum Arg {
    Strings(Vec<String>),
    Files(Vec<FileArg>),
}

impl Arg {
    pub fn new_string_arg(v: Vec<String>) -> Self {
        Arg::Strings(v)
    }

    pub fn new_file_arg(v: Vec<FileArg>) -> Self {
        Arg::Files(v)
    }

    // Panics if the Arg is not a string argument
    fn get_string(&self) -> &[String] {
        match *self {
            Arg::Files(_) => panic!("Could not get_strings, Arg is a file argument"),
            Arg::Strings(ref v) => &v[..],
        }
    }

    // Panics if the Arg is not a file argument
    fn get_file(&self) -> &[FileArg] {
        match *self {
            Arg::Strings(_) => panic!("Could not get_files, Arg is a string argument"),
            Arg::Files(ref v) => &v[..],
        }
    }
}

pub type NodeConstructor = fn(PathBuf) -> Result<IpfsNode, String>;

pub struct Context {
    pub repo_dir: PathBuf,
    pub node: Option<IpfsNode>,
    node_constructor: NodeConstructor,
}

impl Context {
    // takes a path to the repo directory
    pub fn new(path: PathBuf, node_constructor: NodeConstructor) -> Self {
        Context {
            repo_dir: path,
            node: None,
            node_constructor: node_constructor,
        }
    }

    pub fn construct_node(&mut self) -> Result<(), String> {
        let node = try!((self.node_constructor)(self.repo_dir.clone()));
        self.node = Some(node);
        Ok(())
    }
}

// a request for a command to be executed
pub struct Request<'a> {
    pub command: &'a Command,
    arguments: HashMap<super::ArgName, Arg>,
    options: HashMap<super::OptName, Opt>,
    pub context: Context,
}

impl<'a> Request<'a> {
    pub fn new(cmd: &'a Command,
               args: Vec<(super::ArgName, Arg)>,
               opts: Vec<(super::OptName, Opt)>,
               context: Context) -> Self {
        Request {
            command: cmd,
            arguments: args.into_iter().collect(),
            options: opts.into_iter().collect(),
            context: context,
        }
    }

    pub fn args(&self) -> hash_map::Iter<super::ArgName, Arg> {
        self.arguments.iter()
    }

    pub fn string_arg(&self, name: super::ArgName) -> Option<&[String]> {
        self.arguments.get(&name).map(|arg| arg.get_string())
    }

    pub fn file_arg(&self, name: super::ArgName) -> Option<&[FileArg]> {
        self.arguments.get(&name).map(|arg| arg.get_file())
    }

    pub fn options(&self) -> hash_map::Iter<super::OptName, Opt> {
        self.options.iter()
    }

    pub fn option(&self, name: super::OptName) -> Option<&Opt> {
        self.options.get(&name)
    }
}
