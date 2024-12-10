use std::collections::VecDeque;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    Ok(format!("{}", solve(input, true)))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    Ok(format!("{}", solve(input, false)))
  }
}

fn solve(input: String, no_dup: bool) -> usize {
  let map = parse(input);

  let mut sum = 0;
  for (y, line) in map.iter().enumerate() {
    for (x, num) in line.iter().enumerate() {
      if *num == 0 {
        sum += find_paths(&map, (x, y), no_dup);
      }
    }
  }

  sum
}

fn parse(input: String) -> Vec<Vec<u8>> {
  input
    .split("\n")
    .map(|line| {
      line
        .chars()
        .map(|c| format!("{}", c).parse().unwrap())
        .collect()
    })
    .collect()
}

fn find_paths(map: &[Vec<u8>], pos: (usize, usize), no_dup: bool) -> usize {
  let mut queue = VecDeque::new();
  queue.push_back((pos.0 as isize, pos.1 as isize));

  let mut sum = Vec::new();
  while let Some((x, y)) = queue.pop_front() {
    let num = map[y as usize][x as usize];

    for i in -1..=1 {
      if let Some(next) = get_num(map, (x + i, y)) {
        if next - 1 == num && next != 9 {
          queue.push_back((x + i, y));
        } else if next == 9 && next - 1 == num {
          sum.push((x + i, y));
        }
      }
    }

    for i in -1..=1 {
      if let Some(next) = get_num(map, (x, y + i)) {
        if next - 1 == num && next != 9 {
          queue.push_back((x, y + i));
        } else if next == 9 && next - 1 == num {
          sum.push((x, y + i));
        }
      }
    }
  }

  if no_dup {
    sum.sort_unstable();
    sum.dedup();
  }

  sum.len()
}

fn get_num(map: &[Vec<u8>], pos: (isize, isize)) -> Option<u8> {
  if pos.0 < 0 || pos.1 < 0 {
    return None;
  }
  let pos = (pos.0 as usize, pos.1 as usize);

  if let Some(line) = map.get(pos.1) {
    line.get(pos.0).copied()
  } else {
    None
  }
}
