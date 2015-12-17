use commands::{HelpText, Command, Argument};
use commands::request;

use rust_multihash::Multihash;

const ObjectHelpText: HelpText = HelpText {
    tagline: "Interact with ipfs objects",
    synopsis: "",
    short_desc: r#"
'ipfs object' is a plumbing command used to manipulate DAG objects
directly.
"#,
};

const GetHelpText: HelpText = HelpText {
    tagline: "Get and serialize the DAG node named by <key>",
    synopsis: "",
    short_desc: r#"
'ipfs object get' is a plumbing command for retreiving DAG nodes.
It serializes the DAG node to JSON. It outputs to stdout, and <key>
is a base58 encoded multihash.
"#,
};

pub fn make_command() -> Command {
    fn run(req: &mut request::Request) -> Result<(), String> {
        unimplemented!()
    }

    Command::new(vec![],
                 vec![],
                 run,
                 ObjectHelpText,
                 vec![("get", make_get_command())])
}

#[derive(Debug)]
struct Link {
    name: String,
    hash: Multihash,
    size: u64,
}

#[derive(Debug)]
struct Node<'a> {
    links: Vec<Link>,
    data: &'a [u8],
}

fn make_get_command() -> Command {
    let arg_key = Argument::new_string(
        "key",
        true,
        false,
        "Key of the object to retrieve (in base58-encoded multihash format)"
    );

    fn run(req: &mut request::Request) -> Result<(), String> {
        try!(req.context.construct_node());
        let node = req.context.node.as_ref().unwrap();

        let path = &req.string_arg("key").unwrap()[0];
        let mh = try!(Multihash::from_base58_str(path));
        let mut dag_node = try!(node.dagservice.get(&mh));

        let mut links = Vec::new();
        for link in dag_node.get_links() {
            // TODO: no cloning?
            links.push(Link {
                name: link.clone_name(),
                hash: link.clone_hash(),
                size: link.get_target_size(),
            });
        }

        let view_node = Node {
            links: links,
            data: dag_node.get_data(),
        };

        println!("{:?}", view_node);

        Ok(())
    }

    Command::new(vec![], vec![arg_key], run, GetHelpText, vec![])
}
