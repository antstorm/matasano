mod hex;

pub struct ByteArray {
  bytes: Vec<u8>
}

impl ByteArray {
  fn new(hex_string: &str) -> ByteArray {
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

  fn xor(&self, byte: u8) -> ByteArray {
    let mut result = ByteArray { bytes: vec!() };

    for i in self.bytes.iter() {
      let xored = i ^ byte;

      result.bytes.push(xored)
    }

    result
  }

  fn to_string(&self) -> String {
    let mut result = String::new();

    for i in self.bytes.iter() {
      result.push(char::from_u32(*i as u32).unwrap());
    }

    result
  }
}
