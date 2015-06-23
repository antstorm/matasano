pub fn char_distance(a: char, b: char) -> u8 {
  let result = (a as u8) ^ (b as u8);

  return result.count_ones() as u8
}

pub fn hamming_distance(a: &str, b: &str) -> usize {
  let mut result = 0;
  let mut iterator_1 = a.chars();
  let mut iterator_2 = b.chars();

  for _ in 0..a.len() {
    let char_1 = iterator_1.next().unwrap();
    let char_2 = iterator_2.next().unwrap();

    result += char_distance(char_1, char_2) as usize;
  }

  return result
}

fn main() {
  let distance = hamming_distance("this is a test", "wokka wokka!!!");

  println!("Distance = {}", distance);

  assert_eq!(distance, 37);
}
