use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct University {
    pub name: String,
    pub undergraduate_enrollment: u32,
    pub graduate_enrollment: u32,
    pub schools: Vec<String>,
    pub acceptance_rate: f32,
}

pub fn serialize_struct_to_jsonstring(struct_data: &University) -> String {
    serde_json::to_string(struct_data).unwrap()
}

pub fn deserialize_jsonstring_to_struct(string_data: &str) -> University {
    serde_json::from_str(string_data).unwrap()
}

pub fn serialize_struct_to_cbor(struct_data: &University, filename: &str) {
    let mut file = File::create(filename).unwrap();
    serde_cbor::to_writer(&mut file, &struct_data).unwrap();
}

pub fn deserialize_struct_from_cbor(filename: &str) -> University {
    let data = std::fs::read(filename).unwrap();
    serde_cbor::from_slice(&data).unwrap()
}