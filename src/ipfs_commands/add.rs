use commands::{HelpText, Command, Argument};
use commands::request;

const AddHelpText: HelpText = HelpText {
    tagline: "Add an object to ipfs.",
    synopsis: "",
    short_desc: "Adds contents of <path> to ipfs.",
};

pub fn make_command() -> Command {
    let arg_path = Argument::new_file("path",
                                      true,
                                      true,
                                      "The path(s) to a file to be added to \
                                       IPFS");

    fn run(req: &request::Request) -> Result<(), String> {
        for arg in req.args() {
            println!("arg: {:?}", arg);
        }

        unimplemented!()
    }

    Command::new(vec![], vec![arg_path], run, AddHelpText, vec![])
}

// fn add_file<P: AsRef<Path>>(path: P)
