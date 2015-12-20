use commands::{self, HelpText, Command};
use commands::request;
use merkledag::{DagService, Node};
use unixfs::FSNode;

use rust_multihash::Multihash;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

const HELP_TEXT: HelpText = HelpText {
    tagline: "Add an object to ipfs.",
    synopsis: "",
    short_desc: "Adds contents of <path> to ipfs.",
};

fn run(req: &request::Request) -> Result<(), String> {
    let node = try!(req.context.get_node());

    for path in req.file_arg("path").unwrap() {
        let hash = try!(add_file(path, node.dagservice.clone()));
        println!("added {} {:?}", hash, path);
    }
}

ipfs_command!(AddCommand, run);


fn add_file<P: AsRef<Path>>(path: P, ds: Arc<DagService>) -> Result<Multihash, String> {
    let mut file = try!(File::open(path).map_err(|e| {
        format!("Error opening file: {}", e)
    }));
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data);

    let fs_node = FSNode::file_from_bytes(file_data);

    let mut buf = Vec::new();
    try!(fs_node.encode_to_writer(&mut buf));

    let dag_node = Node::from_data(buf);
    ds.add(&dag_node)
}

pub fn make_command() -> Box<Command> {
    let arg_path = commands::Argument::new_file(
        "path",
        true,
        true,
        "The path(s) to a file to be added to IPFS"
    );


    Box::new(AddCommand::new("add", vec![], vec![arg_path], HELP_TEXT, vec![]))
}

// fn add_file<P: AsRef<Path>>(path: P)
