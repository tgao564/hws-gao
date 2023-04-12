use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::Error;

pub fn serialize_data_to_disk(data: HashMap<String, i32>, filename: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    for (key, value) in data {
        let serialized = format!("{} {}\n", key, value);
        file.write_all(serialized.as_bytes())?;
    }
    Ok(())
}

pub fn deserialize_data_from_disk(filename: &str) -> HashMap<String, i32> {
    let file = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("Failed to read file");
    let mut result = HashMap::new();
    for line in contents.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        result.insert(tokens[0].to_string(), tokens[1].parse().unwrap());
    }
    result
}
