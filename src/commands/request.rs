use super::Command;

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

pub struct Context {
    repo_dir: PathBuf,
}

impl Context {
    pub fn new(path: PathBuf) -> Self {
        Context { repo_dir: path }
    }
}

pub struct Request<'a> {
    options: HashMap<super::OptName, Opt>,
    pub command: &'a Command,
    context: Context,
}

impl<'a> Request<'a> {
    pub fn new(opts: Vec<(super::OptName, Opt)>, cmd: &'a Command, context: Context) -> Self {
        Request {
            options: opts.into_iter().collect(),
            command: cmd,
            context: context,
        }
    }

    pub fn options(&self) -> hash_map::Iter<&'static str, Opt> {
        self.options.iter()
    }
}
