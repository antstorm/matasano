// Fixed XOR

// Write a function that takes two equal-length buffers and produces their XOR combination.

// If your function works properly, then when you feed it the string:
// 1c0111001f010100061a024b53535009181c

// ... after hex decoding, and when XOR'd against:
// 686974207468652062756c6c277320657965

// ... should produce:
// 746865206b696420646f6e277420706c6179

mod hex;

fn xor(a: &str, b: &str) -> String {
  let mut a_iterator = a.chars();
  let mut b_iterator = b.chars();
  let mut result = String::new();

  for _ in 0..a.len() {
    let hex_a = hex::to_int(a_iterator.next().unwrap());
    let hex_b = hex::to_int(b_iterator.next().unwrap());
    let a_xor_b = hex_a ^ hex_b;

    result.push(hex::from_int(a_xor_b));
  }

  result
}

fn main() {
  let input = "1c0111001f010100061a024b53535009181c";
  let output = xor(input, "686974207468652062756c6c277320657965");

  println!("result = {}", output);
  assert_eq!(output, "746865206b696420646f6e277420706c6179");
}
