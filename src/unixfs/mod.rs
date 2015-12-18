use protobuf::{self, Message};
use std::io::{Read, Write};

pub mod pb;

struct FSNode {
    data: Vec<u8>,
    blocksizes: Vec<u64>,
    subtotal: u64,
    ty: pb::Data_DataType,
}

impl FSNode {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, String> {
        let mut pb_node = try!(from_reader(reader));
        let data = pb_node.take_Data();
        let data_len = data.len() as u64;
        Ok(FSNode {
            data: data,
            blocksizes: pb_node.take_blocksizes(),
            subtotal: pb_node.get_filesize() - data_len,
            ty: pb_node.get_Type(),
        })
    }

    pub fn encode_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), String> {
        let mut pb_node = pb::Data::new();
        pb_node.set_Data(self.data.clone());
        pb_node.set_Type(self.ty);
        pb_node.set_filesize(self.subtotal + self.data.len() as u64);
        pb_node.set_blocksizes(self.blocksizes.clone());

        pb_node.write_to_writer(writer)
              .map_err(|e| format!("Error cloning node to writer: {}", e))
    }
}

pub fn from_reader<R: Read>(reader: &mut R) -> Result<pb::Data, String> {
    protobuf::parse_from_reader::<pb::Data>(reader)
        .map_err(|e| format!("Error parsing encoded Unixfs data: {}", e))
}
