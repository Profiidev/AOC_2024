use std::collections::HashSet;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let field = parse(input);

    let mut pos = find_pos(&field).unwrap();
    let mut direction = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
      let next_pos = direction.next(pos);
      if !(0..field.len()).contains(&(next_pos.1 as usize))
        || !(0..field[0].len()).contains(&(next_pos.0 as usize))
      {
        break;
      }

      if field[next_pos.1 as usize][next_pos.0 as usize] == '#' {
        direction.turn_90();
      } else {
        pos = next_pos;
        visited.insert(pos);
      }
    }

    Ok(format!("{}", visited.len()))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let field = parse(input);

    let pos = find_pos(&field).unwrap();
    let mut loops = 0;

    for y in 0..field.len() as isize {
      for x in 0..field[0].len() as isize {
        if x == pos.0 && y == pos.1 {
          continue;
        }

        if is_loop(&field, (x, y), pos) {
          loops += 1;
        }
      }
    }

    Ok(format!("{}", loops))
  }
}

fn is_loop(field: &[Vec<char>], obstacle: (isize, isize), start: (isize, isize)) -> bool {
  let mut visited = HashSet::new();
  let mut pos = start;
  let mut direction = Direction::Up;

  loop {
    let next_pos = direction.next(pos);
    if !(0..field.len()).contains(&(next_pos.1 as usize))
      || !(0..field[0].len()).contains(&(next_pos.0 as usize))
    {
      break false;
    }

    if field[next_pos.1 as usize][next_pos.0 as usize] == '#' || obstacle == next_pos {
      direction.turn_90();
    } else {
      pos = next_pos;
      let combination = (pos, direction.clone());

      if !visited.insert(combination) {
        break true;
      }
    }
  }
}

fn parse(input: String) -> Vec<Vec<char>> {
  input
    .split("\n")
    .map(|line| line.chars().collect())
    .collect()
}

fn find_pos(field: &[Vec<char>]) -> Option<(isize, isize)> {
  for (y, line) in field.iter().enumerate() {
    for (x, char) in line.iter().enumerate() {
      if *char == '^' {
        return Some((x as isize, y as isize));
      }
    }
  }
  None
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn next(&self, pos: (isize, isize)) -> (isize, isize) {
    match self {
      Direction::Up => (pos.0, pos.1 - 1),
      Direction::Down => (pos.0, pos.1 + 1),
      Direction::Left => (pos.0 - 1, pos.1),
      Direction::Right => (pos.0 + 1, pos.1),
    }
  }

  fn turn_90(&mut self) {
    match self {
      Direction::Up => *self = Direction::Right,
      Direction::Down => *self = Direction::Left,
      Direction::Left => *self = Direction::Up,
      Direction::Right => *self = Direction::Down,
    }
  }
}
