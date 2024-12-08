use std::collections::{HashMap, HashSet};

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let field = parse_field(input);
    let antennas = parse(&field);

    let mut pos = HashSet::new();

    for (_, positions) in antennas {
      for (i, (x1, y1)) in positions.iter().enumerate() {
        for (x2, y2) in &positions[i + 1..] {
          let diff_x = x1 - x2;
          let diff_y = y1 - y2;

          pos.insert((x1 + diff_x, y1 + diff_y));
          pos.insert((x2 - diff_x, y2 - diff_y));
        }
      }
    }

    pos.retain(|(x, y)| {
      *x >= 0 && *y >= 0 && *x < field[0].len() as isize && *y < field.len() as isize
    });

    Ok(format!("{}", pos.len()))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let field = parse_field(input);
    let antennas = parse(&field);

    let mut pos = HashSet::new();

    for (_, positions) in antennas {
      for (i, (x1, y1)) in positions.iter().enumerate() {
        for (x2, y2) in &positions[i + 1..] {
          let (diff_x, diff_y) = smallest_diff(x1 - x2, y1 - y2);
          if diff_x == 0 && diff_y == 0 {
            panic!("error");
          }

          pos.insert((*x1, *y1));

          let mut current_pos = (*x1, *y1);
          loop {
            let (x, y) = (current_pos.0 - diff_x, current_pos.1 - diff_y);
            if x < 0 || y < 0 {
              break;
            }

            current_pos = (x, y);
            pos.insert(current_pos);
          }

          let mut current_pos = (*x1, *y1);
          loop {
            let (x, y) = (current_pos.0 + diff_x, current_pos.1 + diff_y);
            if x >= field[0].len() as isize || y >= field.len() as isize {
              break;
            }

            current_pos = (x, y);
            pos.insert(current_pos);
          }
        }
      }
    }

    pos.retain(|(x, y)| {
      *x >= 0 && *y >= 0 && *x < field[0].len() as isize && *y < field.len() as isize
    });

    Ok(format!("{}", pos.len()))
  }
}

fn parse_field(input: String) -> Vec<Vec<char>> {
  input
    .split("\n")
    .map(|line| line.chars().collect())
    .collect()
}

fn parse(field: &[Vec<char>]) -> HashMap<char, Vec<(isize, isize)>> {
  let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

  for (y, line) in field.iter().enumerate() {
    for (x, c) in line.iter().enumerate() {
      antennas
        .entry(*c)
        .or_default()
        .push((x as isize, y as isize));
    }
  }

  antennas.into_iter().filter(|(c, _)| *c != '.').collect()
}

fn smallest_diff(mut x: isize, mut y: isize) -> (isize, isize) {
  if x < 0 && y < 0 {
    x = x.abs();
    y = y.abs();
  }

  let larger = if x > y { x } else { y };

  let mut current = (0, 0);
  for i in 1..=larger {
    if x % i == 0 && y % i == 0 {
      current = (x / i, y / i);
    }
  }

  current
}
