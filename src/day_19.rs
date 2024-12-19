use std::collections::HashMap;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let (available, required) = parse(input);

    let possible = required
      .into_iter()
      .filter(|s| check_pattern(s.to_string(), &available))
      .count();

    Ok(format!("{}", possible))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let (available, required) = parse(input);

    let mut cache = HashMap::new();
    let count: usize = required
      .into_iter()
      .map(|s| get_possible_count(s, &available, &mut cache))
      .sum();

    Ok(format!("{}", count))
  }
}

fn get_possible_count(
  pattern: String,
  available: &[String],
  cache: &mut HashMap<String, usize>,
) -> usize {
  let mut sum = 0;
  for s in available {
    if let Some(s) = pattern.strip_prefix(s) {
      if s.is_empty() {
        sum += 1;
      } else if let Some(count) = cache.get(s) {
        sum += *count;
      } else {
        let count = get_possible_count(s.to_string(), available, cache);
        cache.insert(s.to_string(), count);
        sum += count;
      }
    }
  }

  sum
}

fn check_pattern(pattern: String, available: &[String]) -> bool {
  available
    .iter()
    .flat_map(|s| pattern.strip_prefix(s))
    .map(|s| {
      if s.is_empty() {
        true
      } else {
        check_pattern(s.to_string(), available)
      }
    })
    .any(|b| b)
}

fn parse(input: String) -> (Vec<String>, Vec<String>) {
  let mut parts = input.split("\n\n");

  let available: Vec<String> = parts
    .next()
    .unwrap()
    .split(", ")
    .map(|s| s.to_string())
    .collect();

  let required: Vec<String> = parts
    .next()
    .unwrap()
    .split("\n")
    .map(|s| s.to_string())
    .collect();

  (available, required)
}
