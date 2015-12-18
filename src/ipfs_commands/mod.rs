// Defines a type that satisfies the `commands::Command` trait
macro_rules! ipfs_command {
    ($name:ident, $f:ident) => {
        struct $name {
            def: ::commands::CommandDefinition,
        }

        impl $name {
            fn new(name: ::commands::CommandName,
                   options: Vec<::commands::Opt>,
                   arguments: Vec<::commands::Argument>,
                   help_text: ::commands::HelpText,
                   subcommands: Vec<Box<::commands::Command>>)
                   -> Self {
                        $name { def: ::commands::CommandDefinition::new(name,
                                                                        options,
                                                                        arguments,
                                                                        help_text,
                                                                        subcommands) }
                    }
                }

        impl Command for $name {
            fn get_name(&self) -> ::commands::CommandName { self.def.get_name() }

            fn get_help_text(&self) -> &::commands::HelpText { self.def.get_help_text() }

            fn get_options(&self) -> ::commands::CommandOptions { self.def.get_options() }

            fn get_arguments(&self) -> ::std::slice::Iter<::commands::Argument> {
                self.def.get_arguments()
            }

            fn get_subcommand(&self, subcmd: &str) -> Option<&::commands::Command> {
                self.def.get_subcommand(subcmd)
            }

            fn num_args(&self) -> usize { self.def.num_args() }

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
