use serde::{Serialize, Deserialize};
use std::fs::File;


#[derive(Debug, Serialize, Deserialize)]
pub struct University {
    panic!("TODO milestone primer-mod5");
}

pub fn serialize_struct_to_jsonstring(struct_data: &University) -> String {
    panic!("TODO milestone primer-mod5");
}

pub fn deserialize_jsonstring_to_struct(string_data: &str) -> University {
    panic!("TODO milestone primer-mod5");
}



pub fn serialize_struct_to_cbor(struct_data: &University, filename: &str) {
    panic!("TODO milestone primer-mod5");
}

pub fn deserialize_struct_from_cbor(filename: &str) -> University {
    panic!("TODO milestone primer-mod5");
}

