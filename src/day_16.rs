use std::{
  collections::{HashMap, HashSet, VecDeque},
  str::FromStr,
};

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let field = parse(input);
    let start = find_tile(&field, Tile::Start);

    let (cost, _) = path_find(&field, start);

    Ok(format!("{cost}"))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let field = parse(input);
    let start = find_tile(&field, Tile::Start);

    let (_, best) = path_find(&field, start);
    let best: HashSet<(isize, isize)> = best.into_iter().flatten().collect();

    Ok(format!("{}", best.len() + 2))
  }
}

fn path_find(field: &[Vec<Tile>], start: (isize, isize)) -> (usize, Vec<Vec<(isize, isize)>>) {
  let mut queue = VecDeque::new();
  queue.push_back((start, 0, Direction::Right, vec![]));

  let mut best = Vec::new();
  let mut low = usize::MAX;
  let mut seen = HashMap::new();

  while let Some(((x, y), cost, dir, path)) = queue.pop_front() {
    for next in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let mut path = path.clone();

      match field[(y + next.1) as usize][(x + next.0) as usize] {
        Tile::Wall => (),
        Tile::End => {
          if cost < low {
            best.clear();
            low = cost;
          }
          if cost == low {
            best.push(path);
          }
        }
        _ => {
          let (cost_add, dir) = dir.cost(next);
          let cost = cost + cost_add;
          let next_pos = (x + next.0, y + next.1);

          if cost > low {
            continue;
          }
          if *seen.entry((next_pos, dir)).or_insert(cost) < cost {
            continue;
          } else {
            seen.insert((next_pos, dir), cost);
          }

          path.push(next_pos);

          queue.push_back((next_pos, cost, dir, path));
        }
      }
    }
  }

  (low + 1, best)
}

fn find_tile(field: &[Vec<Tile>], tile: Tile) -> (isize, isize) {
  for (y, line) in field.iter().enumerate() {
    for (x, t) in line.iter().enumerate() {
      if *t == tile {
        return (x as isize, y as isize);
      }
    }
  }

  unreachable!()
}

fn parse(input: String) -> Vec<Vec<Tile>> {
  input
    .split("\n")
    .map(|line| line.chars().flat_map(|c| format!("{c}").parse()).collect())
    .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Tile {
  Wall,
  Air,
  Start,
  End,
}

impl FromStr for Tile {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.chars().next() {
      Some('#') => Ok(Tile::Wall),
      Some('.') => Ok(Tile::Air),
      Some('S') => Ok(Tile::Start),
      Some('E') => Ok(Tile::End),
      _ => Err(()),
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn cost(&self, next: (isize, isize)) -> (usize, Self) {
    let possible = [1001, 1, 1001];
    let index = match self {
      Direction::Up | Direction::Down => 1 + next.0,
      Direction::Left | Direction::Right => 1 + next.1,
    };

    let dir = if index == 0 {
      self.left()
    } else if index == 2 {
      self.right()
    } else {
      *self
    };

    (possible[index as usize], dir)
  }

  fn right(&self) -> Self {
    match self {
      Direction::Down => Direction::Left,
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Left => Direction::Up,
    }
  }

  fn left(&self) -> Self {
    match self {
      Direction::Down => Direction::Right,
      Direction::Up => Direction::Left,
      Direction::Right => Direction::Up,
      Direction::Left => Direction::Down,
    }
  }
}
