use protobuf;
use std::io::Read;

pub mod pb;


pub fn from_reader<R: Read>(reader: &mut R) -> Result<pb::Data, String> {
    protobuf::parse_from_reader::<pb::Data>(reader)
        .map_err(|e| format!("Error parsing encoded Unixfs data: {}", e))
}
