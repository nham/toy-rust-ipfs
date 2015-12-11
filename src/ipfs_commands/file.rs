use commands::{HelpText, Command, Argument};
use commands::request;

const FileHelpText: HelpText = HelpText {
    tagline:  "Interact with ipfs objects representing Unix filesystems",
    synopsis: "",
    short_desc: r#"
'ipfs file' provides a familar interface to filesystems represented
by IPFS objects that hides IPFS-implementation details like layout
objects (e.g. fanout and chunking).
"#
};

const LsHelpText: HelpText = HelpText {
    tagline: "List directory contents for Unix-filesystem objects",
    synopsis: "",
    short_desc: r#"
Retrieves the object named by <ipfs-or-ipns-path> and displays the
contents.

The JSON output contains size information.  For files, the child size
is the total size of the file contents.  For directories, the child
size is the IPFS link size.
"#
};

pub fn make_command() -> Command {
    fn run(req: &request::Request)  -> Result<(), String> {
        unimplemented!()
    }

    Command::new(vec![], vec![], run, FileHelpText, vec![("ls", make_ls_command())])
}

fn make_ls_command() -> Command {
    let arg_path = Argument::new_string(
        "ipfs-path",
        true,
        true,
        "The path(s) to the IPFS object(s) to list links from"
    );

    fn run(req: &request::Request)  -> Result<(), String> {
        unimplemented!()
    }

    Command::new(vec![], vec![arg_path], run, FileHelpText, vec![])
}
