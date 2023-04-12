use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::Error;


pub fn serialize_data_to_disk(data: Vec<i32>, filename: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    for d in data {
        let serialized = d.to_string();
        file.write_all(serialized.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

pub fn deserialize_data_from_disk(filename: &str) -> Vec<i32> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    contents.lines().filter_map(|line| line.parse::<i32>().ok()).collect()
}
