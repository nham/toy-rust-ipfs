use commands::{CommandName, Opt, Argument, HelpText, Command, CommandOptions};

use std::collections::HashMap;
use std::slice;

pub struct CommandInfo {
    name: CommandName,
    options: Vec<Opt>,
    arguments: Vec<Argument>,
    help_text: HelpText,
    subcommands: HashMap<CommandName, Box<Command>>,
}

impl CommandInfo {
    fn new(name: CommandName,
           options: Vec<Opt>,
           arguments: Vec<Argument>,
           help_text: HelpText,
           subcommands: Vec<Box<Command>>)
           -> Self {
                CommandInfo {
                    name: name,
                    options: options,
                    arguments: arguments,
                    help_text: help_text,
                    subcommands: subcommands.into_iter()
                                            .map(|cmd| (cmd.get_name(), cmd))
                                            .collect(),
                }
    }

    fn get_name(&self) -> CommandName { self.name }

    fn get_help_text(&self) -> &HelpText { &self.help_text }

    fn get_options(&self) -> CommandOptions {
        CommandOptions::new(self.options.iter())
    }

    fn get_arguments(&self) -> slice::Iter<Argument> {
        self.arguments.iter()
    }

    fn num_args(&self) -> usize { self.arguments.len() }

    fn get_subcommand(&self, subcmd: &str) -> Option<&Command> {
        self.subcommands.get(subcmd).map(|cmd| &**cmd)
    }
}

// Defines a type that satisfies the `commands::Command` trait
macro_rules! ipfs_command {
    ($name:ident, $f:ident) => {
        struct $name {
            info: ::ipfs_commands::CommandInfo,
        }

        impl $name {
            fn new(name: ::commands::CommandName,
                   options: Vec<::commands::Opt>,
                   arguments: Vec<::commands::Argument>,
                   help_text: ::commands::HelpText,
                   subcommands: Vec<Box<::commands::Command>>)
                   -> Self {
                        $name {
                            info: ::ipfs_commands::CommandInfo::new(name, options, arguments, help_text, subcommands)
                        }
                    }
                }

        impl Command for $name {
            fn get_name(&self) -> ::commands::CommandName {
                self.info.get_name()
            }
            fn get_help_text(&self) -> &::commands::HelpText {
                self.info.get_help_text()
            }

            fn get_options(&self) -> ::commands::CommandOptions {
                self.info.get_options()
            }

            fn get_arguments(&self) -> ::std::slice::Iter<::commands::Argument> {
                self.info.get_arguments()
            }

            fn num_args(&self) -> usize {
                self.info.num_args()
            }

            fn get_subcommand(&self, subcmd: &str) -> Option<&::commands::Command> {
                self.info.get_subcommand(subcmd)
            }


            fn run(&self, req: &::commands::request::Request) -> Result<(), String> {
                $f(req)
            }
        }
    }
}

pub mod add;
pub mod file;
pub mod init;
pub mod object;
pub mod root;
