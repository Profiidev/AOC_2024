use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let num: usize = lines
      .iter()
      .enumerate()
      .map(|(i, &l)| {
        l.chars()
          .enumerate()
          .map(|(j, _)| check_xmas(&lines, i, j))
          .sum::<usize>()
      })
      .sum();

    Ok(format!("{}", num))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let num: usize = lines
      .iter()
      .enumerate()
      .map(|(i, &l)| {
        l.chars()
          .enumerate()
          .map(|(j, _)| check_x_mas(&lines, i, j))
          .filter(|b| *b)
          .count()
      })
      .sum();

    Ok(format!("{}", num))
  }
}

fn check_x_mas(lines: &[&str], x: usize, y: usize) -> bool {
  let x = x as isize;
  let y = y as isize;

  if let Some('A') = char_at(lines, x, y) {
    if check_mas(lines, x, y, false) && check_mas(lines, x, y, true) {
      return true;
    }
  }

  false
}

fn check_mas(lines: &[&str], x: isize, y: isize, ul_br: bool) -> bool {
  let modifier = if ul_br { 1 } else { -1 };

  let other = match char_at(lines, x + modifier, y + 1) {
    Some('M') => 'S',
    Some('S') => 'M',
    _ => return false,
  };

  if let Some(c) = char_at(lines, x - modifier, y - 1) {
    if c == other {
      return true;
    }
  }

  false
}

fn check_xmas(lines: &[&str], x: usize, y: usize) -> usize {
  let mut count = 0;
  let x = x as isize;
  let y = y as isize;

  if let Some('X') = char_at(lines, x, y) {
    for i in -1..=1 {
      for j in -1..=1 {
        if let Some('M') = char_at(lines, i + x, j + y) {
          if let Some('A') = char_at(lines, i * 2 + x, j * 2 + y) {
            if let Some('S') = char_at(lines, i * 3 + x, j * 3 + y) {
              count += 1;
            }
          }
        }
      }
    }
  }

  count
}

fn char_at(lines: &[&str], x: isize, y: isize) -> Option<char> {
  if x < 0 || y < 0 {
    None
  } else {
    lines.get(x as usize)?.chars().nth(y as usize)
  }
}
