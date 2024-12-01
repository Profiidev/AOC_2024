use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    let sum: i32 = left
      .into_iter()
      .zip(right)
      .map(|(l, r)| (l - r).abs())
      .sum();
    Ok(format!("{}", sum))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let (left, right) = parse(input);
    let sum: i32 = left
      .iter()
      .map(|l| right.iter().filter(|&r| r == l).count() as i32 * (*l))
      .sum();
    Ok(format!("{}", sum))
  }
}

fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
  input
    .split("\n")
    .map(|pair| {
      let mut parts = pair.split("   ");
      (parts.next().unwrap(), parts.next().unwrap())
    })
    .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
    .unzip()
}
