use commands::{HelpText, Command, Argument};
use commands::request;
use unixfs;

use rust_multihash::Multihash;
use std::collections::HashMap;
use std::sync::Arc;

const FileHelpText: HelpText = HelpText {
    tagline: "Interact with ipfs objects representing Unix filesystems",
    synopsis: "",
    short_desc: r#"
'ipfs file' provides a familar interface to filesystems represented
by IPFS objects that hides IPFS-implementation details like layout
objects (e.g. fanout and chunking).
"#,
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
"#,
};

pub fn make_command() -> Command {
    fn run(req: &request::Request) -> Result<(), String> {
        unimplemented!()
    }

    Command::new(vec![],
                 vec![],
                 run,
                 FileHelpText,
                 vec![("ls", make_ls_command())])
}

#[derive(Debug)]
struct LsLink {
    pub name: String,
    hash: Multihash,
    size: u64,
    ty: unixfs::pb::Data_DataType,
}

#[derive(Debug)]
struct LsObject {
    hash: Multihash,
    size: u64,
    ty: unixfs::pb::Data_DataType,
    pub links: Vec<LsLink>,
}

fn make_ls_command() -> Command {
    let arg_path = Argument::new_string("ipfs-path",
                                        true,
                                        true,
                                        "The path(s) to the IPFS object(s) \
                                         to list links from");

    // TODO: this is only going to accept hashes for now. Need to implement
    // path resolver so it can do paths.
    fn run(req: &request::Request) -> Result<(), String> {
        let node = try!(req.context.get_node());

        let mut objects: HashMap<Multihash, LsObject> = HashMap::new();

        for path in req.string_arg("ipfs-path").unwrap() {
            let mh = try!(Multihash::from_base58_str(&path));
            // retrieve merkledag node for the path (multihash, at this point)
            let mut dag_node = try!(node.dagservice.get(&mh));
            let unixfs_data =
                try!(unixfs::from_reader(&mut dag_node.get_data()));

            let file_type = unixfs_data.get_Type();

            let links = match file_type {
                unixfs::pb::Data_DataType::File => vec![],
                unixfs::pb::Data_DataType::Directory => {
                    let links = Arc::get_mut(&mut dag_node)
                                    .unwrap()
                                    .get_mut_links();
                    let mut v = Vec::with_capacity(links.len());

                    for link in links.iter_mut() {
                        let link_node = try!(link.get_node(&node.dagservice));
                        link.set_node(link_node.clone()); // TODO: needed?

                        let link_node_data = try!(unixfs::from_reader(&mut link_node.get_data()));

                        let ty = link_node_data.get_Type();

                        let size = match ty {
                            unixfs::pb::Data_DataType::File => {
                                link_node_data.get_filesize()
                            }
                            _ => link.get_target_size(),
                        };

                        v.push(LsLink {
                            name: link.clone_name(),
                            hash: link.clone_hash(),
                            size: size,
                            ty: ty,
                        });
                    }

                    v
                }
                _ => unimplemented!(),
            };

            let ls_obj = LsObject {
                hash: mh,
                size: unixfs_data.get_filesize(),
                ty: file_type,
                links: links,
            };

            objects.insert(dag_node.multihash(), ls_obj);
        }

        for (hash, obj) in &objects {
            println!("{}:", hash);
            for link in obj.links.iter() {
                println!("{}", link.name);
            }
            println!("");
        }
        Ok(())
    }

    Command::new(vec![], vec![arg_path], run, FileHelpText, vec![])
}
