// Single-byte XOR cipher

// The hex encoded string:
// 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
// ... has been XOR'd against a single character. Find the key, decrypt the message.

// You can do this by hand. But don't: write code to do it for you.
// How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.

// Achievement Unlocked
// You now have our permission to make "ETAOIN SHRDLU" jokes on Twitter.

mod hex;
mod byte_array;
mod english_probability;

fn find_key(input: &str) -> u8 {
  let input_bytes = byte_array::ByteArray::new(input);
  let mut scores = vec!();

  for i in 0..256 {
    let result = input_bytes.xor(&[i as u8]).to_string();

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

  max_position as u8
}

fn main() {
  let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  let output = find_key(input);

  println!("result = {}", output);
  assert_eq!(output, 88);
}
