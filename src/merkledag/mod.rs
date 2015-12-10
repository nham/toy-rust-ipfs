mod pb;

use util;

use rust_multihash::Multihash;
use std::io::Read;
use protobuf;

// TODO: pointer to node. How to do?
struct Link {
    name: String,
    hash: Multihash,
    target_size: u64,
}

struct Node {
    data: Vec<u8>,
    links: Vec<Link>,
}

impl Link {
    pub fn from_pblink(mut link: pb::PBLink) -> Self {
        Link {
            name: link.take_Name(),
            hash: util::hash(link.get_Hash()),
            target_size: link.get_Tsize(),
        }
    }
}

impl Node {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, String> {
        let mut pbnode = try!(protobuf::parse_from_reader::<pb::PBNode>(reader)
                              .map_err(|e| format!("Error parsing encoded Node: {}", e)));

        let mut links = Vec::new();

        for link in pbnode.take_Links().into_iter() {
            links.push(Link::from_pblink(link));
        }

        Ok(Node {
            data: pbnode.take_Data(),
            links: links,
        })
    }
}
