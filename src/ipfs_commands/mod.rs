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
            fn get_def(&self) -> &::commands::CommandDefinition { &self.def }

            fn run(&self, req: &request::Request) -> Result<(), String> {
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
