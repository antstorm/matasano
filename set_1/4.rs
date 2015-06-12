use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod hex;
mod byte_array;
mod english_probability;

fn find_key_and_score(input: &str) -> (u8, u8) {
  let input_bytes = byte_array::ByteArray::new(input);
  let mut scores = vec!();

  for i in 0..256 {
    let result = input_bytes.xor(i as u8).to_string();

    scores.push(english_probability::EnglishProbability::score_for(&result));
  }

  let mut max_value = 0;
  let mut max_position = 0;

  for (index, value) in scores.iter().enumerate() {
    if *value > max_value {
      max_value = *value;
      max_position = index;
    }
  }

  (max_position as u8, max_value as u8)
}

fn main() {
  let mut max_score = 0;
  let mut result = String::new();

  let input_file = match File::open("4.txt") {
    Err(error) => panic!("error while opening file: {}", Error::description(&error)),
    Ok(file) => file
  };

  let buffer = BufReader::new(input_file);
  for line in buffer.lines() {
    let input_line = match line {
      Err(error) => panic!("error reading file: {}", Error::description(&error)),
      Ok(line) => line
    };

    let (key, score) = find_key_and_score(&input_line);

    if score > max_score {
      let input_bytes = byte_array::ByteArray::new(&input_line);
      max_score = score;
      result = input_bytes.xor(key as u8).to_string();
    }
  }

  println!("result = {}", result);
  assert_eq!(result, "Now that the party is jumping\n");
}
