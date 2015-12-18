use commands::{self, HelpText, Command, CommandDefinition};
use commands::request;
use super::*;

const HELP_TEXT: HelpText = HelpText {
    tagline: "Add an object to ipfs.",
    synopsis: "",
    short_desc: "Adds contents of <path> to ipfs.",
};

fn run(req: &request::Request) -> Result<(), String> {
    for arg in req.args() {
        println!("arg: {:?}", arg);
    }

    unimplemented!()
}

ipfs_command!(AddCommand, run);


pub fn make_command() -> Box<Command> {
    let arg_path = commands::Argument::new_file(
        "path",
        true,
        true,
        "The path(s) to a file to be added to IPFS"
    );


    let def = CommandDefinition::new("add", vec![], vec![arg_path], HELP_TEXT, vec![]);
    Box::new(AddCommand::new(def))
}

// fn add_file<P: AsRef<Path>>(path: P)
