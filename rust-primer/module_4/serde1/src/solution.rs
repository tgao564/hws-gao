use std::io::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

// Serializes an integer into a string
// 1. what's the difference between casting into a string and serializing into a string?
pub fn serialize_to_string(data: u32) -> String {
    data.to_string()
}

/// Serializes an integer into bytes
pub fn serialize_to_bytes(data: u32) -> [u8; 4] {
    data.to_be_bytes()
}

/// Reads the contents of a file and deserializes them into an integer
pub fn deserialize_from_bytes(bytes:  [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}
