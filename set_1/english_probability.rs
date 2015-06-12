use std::collections::BTreeMap;

pub struct EnglishProbability;

impl EnglishProbability {
  pub fn score_for(input: &str) -> i16 {
    use std::ascii::AsciiExt;

    let perfect_probability = "etaoin shrdlu";
    let mut letter_scores = BTreeMap::new();
    let mut total_score: i16 = 0;

    for i in input.chars() {
      if let Some(_) = perfect_probability.find(i) {
        *letter_scores.entry(i.to_ascii_lowercase()).or_insert(0) += 1;
      }
    }

    let mut popular_letters = vec!();

    for (letter, score) in letter_scores.iter() {
      if *score > 1 {
        popular_letters.push(*letter);
      }
    }
    popular_letters.sort_by(|a, b| letter_scores[b].cmp(&letter_scores[a]));

    for (_, letter) in popular_letters.iter().enumerate() {
      if let Some(_) = perfect_probability.find(*letter as char) {
        total_score += 1;
      }
    }

    total_score
  }
}
