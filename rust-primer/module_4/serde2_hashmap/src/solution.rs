use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::Error;

pub fn serialize_data_to_disk(data: HashMap<String, i32>, filename: &str) -> Result<(), Error> {
    panic!("TODO milestone primer-mod5");
}

pub fn deserialize_data_from_disk(filename: &str) -> HashMap<String, i32> {
    panic!("TODO milestone primer-mod5");
}
