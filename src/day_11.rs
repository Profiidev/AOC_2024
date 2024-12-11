use std::collections::HashMap;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let nums = parse(input);

    let mut cache = HashMap::new();
    let res: usize = nums
      .iter()
      .map(|num| compute(*num, 0, 25, &mut cache))
      .sum();

    Ok(format!("{}", res))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let nums = parse(input);

    let mut cache = HashMap::new();
    let res: usize = nums
      .iter()
      .map(|num| compute(*num, 0, 75, &mut cache))
      .sum();

    Ok(format!("{}", res))
  }
}

fn parse(input: String) -> Vec<u64> {
  input.split(" ").map(|num| num.parse().unwrap()).collect()
}

fn compute(num: u64, depth: u32, max_depth: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
  if depth == max_depth {
    return 1;
  }

  if let Some(res) = cache.get(&(num, depth)) {
    return *res;
  }

  let ret = if num == 0 {
    compute(1, depth + 1, max_depth, cache)
  } else if num.to_string().len() % 2 == 0 {
    let num_str = num.to_string();
    let (left, right) = num_str.split_at(num.to_string().len() / 2);
    compute(left.parse().unwrap(), depth + 1, max_depth, cache)
      + compute(right.parse().unwrap(), depth + 1, max_depth, cache)
  } else {
    compute(num * 2024, depth + 1, max_depth, cache)
  };

  cache.insert((num, depth), ret);

  ret
}
