use super::{Command, OptType};
use super::request::{self, Request};

use std::collections::HashMap;
use std::slice;

pub type ParseError = String;
pub type ParseResult<'a> = (&'a Command,
                            Vec<(super::ArgName, request::Arg)>,
                            Vec<(super::OptName, request::Opt)>);
// TODO: support command arguments
// TODO: support setting option arguments with equal sign (--opt=value)
pub fn parse<I>(mut input: I, root: &Command) -> Result<ParseResult, ParseError>
    where I: Iterator<Item = String>
{
    // As the command-line request is parsed, current_cmd stores the last
    // component of the command path. I.e. if we've parsed
    //
    //     <cmd1> <cmd2> --opt1 --opt2 <cmd3>
    //
    // Then current_cmd will be a reference to (the Command associated with) <cmd3>
    let mut current_cmd = root;

    // Options submitted in the request
    let mut opts: Vec<(super::OptName, request::Opt)> = Vec::new();

    // Arguments submitted in the request
    let mut args: Vec<(super::ArgName, request::Arg)> = Vec::new();

    // Arguments for a single command argument (arguments can be variadic)
    let mut args_one = Vec::new();

    // Stores all the command options for each command in the command path
    let mut cmd_opts = HashMap::new();
    cmd_opts.extend(root.get_options());

    // Stores the command args when we encounter a command with arguments
    let mut cmd_args: Option<slice::Iter<super::Argument>> = None;
    // used for keeping track of which command arg is currently being parsed
    let mut curr_arg = None;

    let mut token: String;

    loop {
        token = match input.next() {
            None => break,
            Some(s) => s,
        };

        // TODO: doesn't parse short args correctly, for now
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
                OptType::Bool => {
                    opts.push((cmd_opt.get_name(), request::Opt::Bool(true)))
                }
                _ => {
                    // just assume the option argument is the next token.
                    // eventually this will check if theres an equal sign
                    // and split the token based on that
                    token = match input.next() {
                        None => {
                            return Err(format!("Expecting option argument for option \
                                                {}, but no more tokens.",
                                               cmd_opt.get_name()))
                        }
                        Some(s) => s,
                    };

                    let req_opt = try!(request::Opt::parse_string(token,
                                                                  cmd_opt.opt_type));
                    opts.push((cmd_opt.get_name(), req_opt));
                }
            }

        } else {
            // Once current_cmd points to a Command with arguments, stop trying to parse
            // sub commands and parse arguments instead. (We still parse options, but
            // any token that isn't an option (i.e. doesn't start with - or --) will
            // be assumed to be an argument)
            let num_args = current_cmd.num_args();
            if num_args == 0 {
                let subcmd = match current_cmd.get_subcommand(&token) {
                    None => return Err(format!("Subcommand {} not found", &token)),
                    Some(cmd) => cmd,
                };

                cmd_opts.extend(subcmd.get_options());
                current_cmd = subcmd;

                // initialize cmd_args if necessary
                let num_args = current_cmd.num_args();
                if num_args > 0 {
                    cmd_args = Some(current_cmd.get_arguments());
                    curr_arg = cmd_args.as_mut().unwrap().next();
                }
            } else {
                // TODO: handle optional arguments

                // Command arg index has been incremented past the end of cmd_args slice
                if curr_arg.is_none() {
                    return Err(format!("Unexpected argument: {}", token));
                }

                args_one.push(token);

                // If it isn't variadic, there are no more arguments to parse, so move
                // onto the next
                if !curr_arg.unwrap().is_variadic() {
                    args.push((curr_arg.unwrap().name(),
                               try!(parse_arg_tokens(curr_arg.unwrap(), args_one))));
                    args_one = Vec::new();
                    curr_arg = cmd_args.as_mut().unwrap().next();
                }
            }
        }
    }

    // Finish parsing args
    if args_one.len() > 0 {
        args.push((curr_arg.unwrap().name(),
                   try!(parse_arg_tokens(curr_arg.unwrap(), args_one))));
        curr_arg = cmd_args.as_mut().unwrap().next();
    }


    if !curr_arg.is_none() {
        return Err(format!("Missing argument for <{}>", curr_arg.unwrap().name()));
    }

    Ok((current_cmd, args, opts))
}

fn parse_arg_tokens(cmd_arg: &super::Argument,
                    args: Vec<String>)
                    -> Result<request::Arg, String> {
    match cmd_arg.arg_type() {
        super::ArgumentType::String => Ok(request::Arg::new_string_arg(args)),

        super::ArgumentType::File => {
            let mut file_args = Vec::new();
            for arg in args.into_iter() {
                let file_arg = try!(request::FileArg::new(arg));
                file_args.push(file_arg);
            }
            Ok(request::Arg::new_file_arg(file_args))
        }
    }
}
