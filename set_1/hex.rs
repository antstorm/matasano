pub struct Hex;

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
