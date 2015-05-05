fn hex_to_int(hex: char) -> u8 {
  let mut unicode_scalar = hex as u8;

  // 48 => "0", 97 => "a"
  unicode_scalar -= 48;
  if unicode_scalar > 9 { unicode_scalar -= 39; }

  unicode_scalar
}

fn int_to_hex(num: u8) -> char {
  let result: u8;

  if num < 10 {
    result = num + 48;
  } else {
    result = num + 87;
  }

  result as char
}

fn xor(a: &str, b: &str) -> String {
  let mut a_iterator = a.chars();
  let mut b_iterator = b.chars();
  let mut result = String::new();

  for _ in 0..a.len() {
    let hex_a = hex_to_int(a_iterator.next().unwrap());
    let hex_b = hex_to_int(b_iterator.next().unwrap());
    let a_xor_b = hex_a ^ hex_b;

    result.push(int_to_hex(a_xor_b));
  }

  result
}

fn main() {
  let input = "1c0111001f010100061a024b53535009181c";
  let output = xor(input, "686974207468652062756c6c277320657965");

  println!("result = {}", output);
  assert_eq!(output, "746865206b696420646f6e277420706c6179");
}
