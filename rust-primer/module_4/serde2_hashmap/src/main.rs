use std::collections::HashMap;
use solution::{serialize_data_to_disk, deserialize_data_from_disk};

fn main() {
    println!("Serializing and Deserializing hashmaps");
    // Change this variable to serialize data (true) or deserialize it (false)
    let serialize = true;
    let filename = "data.bin";
    let data: HashMap<String, i32> = HashMap::from([
        ("Mercury".to_string(), 4),
        ("Venus".to_string(), 7),
        ("Earth".to_string(), 0),
        ("Mars".to_string(), 5),
    ]);
    if serialize {
        serialize_data_to_disk(data, &filename).unwrap();
    } else {
        let deserialized_data = deserialize_data_from_disk(&filename);
        println!("The size of the hashmap is: {}", deserialized_data.len());
        println!("This is the data:");
        for (key, value) in &deserialized_data {
            println!("{}: {}", key, value);
        }
        assert_eq!(data == deserialized_data, true);
    }
}
