macro_rules! ipfs_command {
    ($name:ident, $f:ident) => {
        struct $name {
            def: CommandDefinition,
        }

        impl $name {
            fn new(def: CommandDefinition) -> $name {
                $name { def: def }
            }
        }

        impl Command for $name {
            fn get_def(&self) -> &CommandDefinition { &self.def }

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
