pub struct EnglishProbability;

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
