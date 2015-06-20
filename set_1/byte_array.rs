use std::char;

use hex;

pub struct ByteArray {
  bytes: Vec<u8>
}

impl ByteArray {
  pub fn new(hex_string: &str) -> ByteArray {
    let mut bytes: Vec<u8> = vec!();
    let mut iterator = hex_string.chars();
    let half_length = hex_string.len() / 2;

    for _ in 0..half_length {
      let first_bits = hex::to_int(iterator.next().unwrap());
      let last_bits = hex::to_int(iterator.next().unwrap());

      bytes.push((first_bits << 4) + last_bits);
    }

    ByteArray { bytes: bytes }
  }

  pub fn xor(&self, bytes: &[u8]) -> ByteArray {
    let mut result = ByteArray { bytes: vec!() };
    let mut current_byte_index = 0;

    for i in self.bytes.iter() {
      let byte = bytes[current_byte_index];

      current_byte_index += 1;
      if current_byte_index >= bytes.len() - 1 {
        current_byte_index = 0;
      }

      let xored = i ^ byte;

      result.bytes.push(xored)
    }

    result
  }

  pub fn to_string(&self) -> String {
    let mut result = String::new();

    for i in self.bytes.iter() {
      result.push(char::from_u32(*i as u32).unwrap());
    }

    result
  }
}
