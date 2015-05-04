fn hex_to_int(hex: char) -> u8 {
  let mut unicode_scalar = hex as u8;

  // 48 => "0", 97 => "a"
  unicode_scalar -= 48;
  if unicode_scalar > 9 { unicode_scalar -= 39; }

  unicode_scalar
}

fn int_to_base64(num: u8) -> char {
  let result: u8;

  if num < 26 {
    result = num + 65;
  } else if num < 52 {
    result = num + 71;
  } else if num < 62 {
    result = num - 4;
  } else if num == 62 {
    result = '+' as u8;
  } else {
    result = '/' as u8;
  }

  result as char
}

fn hex_to_base64(hex: &str) -> String {
  let mut result = String::new();
  let mut digit_iterator = hex.chars();

  // 3 4-bit digits at a time
  // 4 bits * 3 = 12 bits = 2 base64 digits
  for _ in 0..(hex.len() / 3) {
    let left_part = hex_to_int(digit_iterator.next().unwrap());
    let middle_part = hex_to_int(digit_iterator.next().unwrap());
    let right_part = hex_to_int(digit_iterator.next().unwrap());

    // 4 bits from the left part and first 2 bits from the middle part
    let first_digit = (left_part << 2) + (middle_part >> 2);
    // last 2 bits from the middle part and 4 bits from the right part
    let last_digit = ((middle_part & 0x3) << 4) + right_part;

    result.push(int_to_base64(first_digit));
    result.push(int_to_base64(last_digit));
  }

  result
}



fn main() {
  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let output = hex_to_base64(input);
  let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  println!("result = {}", output);
  assert_eq!(output, expected);
}
