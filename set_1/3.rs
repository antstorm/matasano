use std::char;
use std::collections::BTreeMap;

struct Hex;

impl Hex {
  fn to_int(hex: char) -> u8 {
    let mut unicode_scalar = hex as u8;

    // 48 => "0", 97 => "a"
    unicode_scalar -= 48;
    if unicode_scalar > 9 { unicode_scalar -= 39; }

    unicode_scalar
  }

  // fn from_int(num: u8) -> char {
  //   let result: u8;

  //   if num < 10 {
  //     result = num + 48;
  //   } else {
  //     result = num + 87;
  //   }

  //   result as char
  // }
}

struct ByteArray {
  bytes: Vec<u8>
}

impl ByteArray {
  fn new(hex_string: &str) -> ByteArray {
    let mut bytes: Vec<u8> = vec!();
    let mut iterator = hex_string.chars();
    let half_length = hex_string.len() / 2;

    for _ in 0..half_length {
      let first_bits = Hex::to_int(iterator.next().unwrap());
      let last_bits = Hex::to_int(iterator.next().unwrap());

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

struct EnglishProbability;

impl EnglishProbability {
  fn score_for(input: &str) -> i16 {
    use std::ascii::AsciiExt;

    let perfect_probability = "etaoinshrdlu";
    let mut letter_scores = BTreeMap::new();
    let mut total_score: i16 = 0;

    for i in input.chars() {
      if i.is_alphabetic() {
        *letter_scores.entry(i.to_ascii_lowercase()).or_insert(0) += 1;
      }
    }

    let mut letters_sorted_by_score = vec!();

    for letter in letter_scores.keys() { letters_sorted_by_score.push(*letter); }
    letters_sorted_by_score.sort_by(|a, b| letter_scores[b].cmp(&letter_scores[a]));

    for (index, letter) in letters_sorted_by_score.iter().enumerate() {
      if let Some(position) = perfect_probability.find(*letter as char) {
        let perfect_score = perfect_probability.len() - position;
        let reverse_index = letters_sorted_by_score.len() - index;

        total_score += (perfect_score * reverse_index) as i16;
      }
    }

    total_score
  }
}

fn find_key(input: &str) -> u8 {
  let input_bytes = ByteArray::new(input);
  let mut scores = vec!();

  for i in 0..256 {
    let result = input_bytes.xor(i as u8).to_string();

    scores.push(EnglishProbability::score_for(&result));
  }

  let mut max_value = 0;
  let mut max_position = 0;

  for (index, value) in scores.iter().enumerate() {
    if *value > max_value {
      max_value = *value;
      max_position = index;
    }
  }

  max_position as u8
}

fn main() {
  let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  let output = find_key(input);

  println!("result = {}", output);
  assert_eq!(output, 88);
}
