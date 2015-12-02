use super::{Command, OptType};
use super::request::{self, Request};

use std::collections::HashMap;

pub type ParseError = String;

// TODO: support command arguments
// TODO: support setting option arguments with equal sign (--opt=value)
pub fn parse<I>(mut input: I, root: &Command)
            -> Result<(Request, &Command), ParseError>
    where I : Iterator<Item=String>
{
    /*
     * loop through all the tokens in the input, gathering up the options
     * and their associated values. also get the subcommand
     *
     */
    let mut current_cmd = root;
    let mut opts: Vec<(&'static str, request::Opt)> = Vec::new();
    let mut cmd_opts = HashMap::new();
    cmd_opts.extend(root.options());

    let mut token: String;
    loop {
        token = match input.next() {
            None => break,
            Some(s) => s,
        };

        // don't parse short args correctly, for now
        // you should be able to combine them, like -a -b -c can be written
        // -abc, with the caveat that all but the last option need to be boolean
        if token.starts_with("--") || token.starts_with("-") {
            let cmd_opt = {
                let opt_name = if token.starts_with("--") {
                    &token[2..]
                } else {
                    &token[1..]
                };

                match cmd_opts.get(opt_name) {
                    None => return Err(format!("Option not recognized: {}", opt_name)),
                    Some(opt) => opt,
                }
            };

            match cmd_opt.opt_type {
                OptType::Bool => opts.push((cmd_opt.name(), request::Opt::Bool(true))),
                _ => {
                    // just assume the option argument is the next token.
                    // eventually this will check if theres an equal sign
                    // and split the token based on that
                    token = match input.next() {
                        None => return Err(format!("Expecting option argument for option \
                                                    {}, but no more tokens.", cmd_opt.name())),
                        Some(s) => s,
                    };

                    let req_opt = try!(request::Opt::parse_string(token,
                                                                  cmd_opt.opt_type));
                    opts.push((cmd_opt.name(), req_opt));
                }
            }

        } else {
            // no arguments for now, so it must be a subcommand!
            let subcmd = match current_cmd.subcommand(&token) {
                None => return Err(format!("Subcommand {} not found", &token)),
                Some(cmd) => cmd,
            };

            cmd_opts.extend(subcmd.options());
            current_cmd = subcmd;
        }
    }
    Ok((Request::new(opts, current_cmd), current_cmd))
}
