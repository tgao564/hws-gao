use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct University {
    name: String,
    location: String,
    ranking: u32,
}

pub fn serialize_struct_to_jsonstring(struct_data: &University) -> String {
    serde_json::to_string(struct_data).unwrap()
}

pub fn deserialize_jsonstring_to_struct(string_data: &str) -> University {
    serde_json::from_str(string_data).unwrap()
}

pub fn serialize_struct_to_cbor(struct_data: &University, filename: &str) -> Result<(), Error> {
    let file = File::create(filename)?;
    serde_cbor::to_writer(file, struct_data);
    Ok(())
}

pub fn deserialize_struct_from_cbor(filename: &str) -> Result<University, Error> {
    unimplemented!()
}

