use std::io::Read;

pub fn build_dag_from_reader<R: Read>(reader: &mut R, ds: 
