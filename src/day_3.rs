use anyhow::Error;
use regex::Regex;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
    let matches = regex.captures_iter(&input);
    let sum: i32 = matches
      .into_iter()
      .map(|i| i.get(0).unwrap().as_str())
      .map(|mul| {
        let nums = mul.replace("mul(", "").replace(")", "");
        let parts = nums.split(",").collect::<Vec<&str>>();
        (
          parts[0].parse::<i32>().unwrap(),
          parts[1].parse::<i32>().unwrap(),
        )
      })
      .map(|(a, b)| a * b)
      .sum();

    Ok(format!("{}", sum))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
    let do_reg = Regex::new(r"(do\(\)|don't\(\))")?;

    let matches = regex.captures_iter(&input);
    let do_matches = do_reg.captures_iter(&input);

    let enable_iter = do_matches
      .into_iter()
      .map(|c| {
        let cap = c.get(0).unwrap();
        (cap.as_str() == "do()", cap.end())
      })
      .collect::<Vec<(bool, usize)>>();

    let sum: i32 = matches
      .into_iter()
      .map(|i| i.get(0).unwrap())
      .filter_map(|mat| {
        let do_before = enable_iter
          .iter()
          .filter(|(_, pos)| *pos <= mat.start())
          .last()
          .cloned()
          .unwrap_or((true, 0));
        if !do_before.0 {
          return None;
        }

        let nums = mat.as_str().replace("mul(", "").replace(")", "");
        let parts = nums.split(",").collect::<Vec<&str>>();
        Some((
          parts[0].parse::<i32>().unwrap(),
          parts[1].parse::<i32>().unwrap(),
        ))
      })
      .map(|(a, b)| a * b)
      .sum();

    Ok(format!("{}", sum))
  }
}
