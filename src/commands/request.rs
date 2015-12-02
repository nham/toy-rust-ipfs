use super::Command;
use super::OptType as CommandOptType;

use std::collections::HashMap;
use std::collections::hash_map;

// An option submitted for a request.
#[derive(Debug)]
pub enum Opt {
    String(String),
    Bool(bool),
    Int(i32),
}

impl Opt {
    pub fn parse_string(s: String, opt_type: CommandOptType) -> Result<Self, String> {
        match opt_type {
            CommandOptType::String => Ok(Opt::String(s)),
            CommandOptType::Bool => {
                if s == "true" {
                    Ok(Opt::Bool(true))
                } else if s == "false" {
                    Ok(Opt::Bool(false))
                } else {
                    Err("Expected boolean value for boolean option.".to_string())
                }
            }
            CommandOptType::Int => {
                let x = try!(s.parse::<i32>()
                              .map_err(|e| format!("{}", e)));
                Ok(Opt::Int(x))
            }
        }
    }
}

pub struct Request<'a> {
    options: HashMap<&'static str, Opt>,
    pub command: &'a Command,
}

impl<'a> Request<'a> {
    pub fn new(opts: Vec<(&'static str, Opt)>, cmd: &'a Command) -> Self {
        Request {
            options: opts.into_iter().collect(),
            command: cmd,
        }
    }

    pub fn options(&self) -> hash_map::Iter<&'static str, Opt> {
        self.options.iter()
    }
}
