// Defines a type that satisfies the `commands::Command` trait
macro_rules! ipfs_command {
    ($name:ident, $f:ident) => {
        struct $name {
            name: ::commands::CommandName,
            options: Vec<::commands::Opt>,
            arguments: Vec<::commands::Argument>,
            help_text: ::commands::HelpText,
            subcommands: ::std::collections::HashMap<::commands::CommandName, Box<::commands::Command>>,
        }

        impl $name {
            fn new(name: ::commands::CommandName,
                   options: Vec<::commands::Opt>,
                   arguments: Vec<::commands::Argument>,
                   help_text: ::commands::HelpText,
                   subcommands: Vec<Box<::commands::Command>>)
                   -> Self {
                        $name {
                            name: name,
                            options: options,
                            arguments: arguments,
                            help_text: help_text,
                            subcommands: subcommands.into_iter()
                                                    .map(|cmd| (cmd.get_name(), cmd))
                                                    .collect(),
                        }
                    }
                }

        impl Command for $name {
            fn get_name(&self) -> ::commands::CommandName { self.name }
            fn get_help_text(&self) -> &::commands::HelpText { &self.help_text }

            fn get_options(&self) -> ::commands::CommandOptions {
                ::commands::CommandOptions::new(self.options.iter())
            }

            fn get_arguments(&self) -> ::std::slice::Iter<::commands::Argument> {
                self.arguments.iter()
            }

            fn num_args(&self) -> usize { self.arguments.len() }

            fn get_subcommand(&self, subcmd: &str) -> Option<&::commands::Command> {
                self.subcommands.get(subcmd).map(|cmd| &**cmd)
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
