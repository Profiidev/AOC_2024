use std::str::FromStr;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let (mut field, movements) = parse(input);
    let mut pos = find_robot(&field);

    for dir in movements {
      movement(&mut field, &mut pos, dir);
    }

    let sum: usize = get_gps(&field).into_iter().map(|(x, y)| y * 100 + x).sum();

    Ok(format!("{}", sum))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let (field, movements) = parse(input);
    let mut field = double_wide(field);
    let mut pos = find_robot(&field);

    for dir in movements {
      movement2(&mut field, &mut pos, dir);
    }

    let sum: usize = get_gps(&field).into_iter().map(|(x, y)| y * 100 + x).sum();

    Ok(format!("{}", sum))
  }
}

fn is_moveable(field: &[Vec<Tile>], pos: (isize, isize), dir: (isize, isize)) -> bool {
  match field[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] {
    Tile::Wall => false,
    Tile::Air => true,
    Tile::BoxLeft => {
      if dir.1 != 0 {
        is_moveable(field, (pos.0, pos.1 + dir.1), dir)
          && is_moveable(field, (pos.0 + 1, pos.1 + dir.1), dir)
      } else {
        is_moveable(field, (pos.0 + 2, pos.1), dir)
      }
    }
    Tile::BoxRight => {
      if dir.1 != 0 {
        is_moveable(field, (pos.0 - 1, pos.1 + dir.1), dir)
          && is_moveable(field, (pos.0, pos.1 + dir.1), dir)
      } else {
        is_moveable(field, (pos.0 - 2, pos.1), dir)
      }
    }
    _ => unreachable!(),
  }
}

fn apply_move(field: &mut [Vec<Tile>], pos: (isize, isize), dir: (isize, isize)) {
  match field[pos.1 as usize][pos.0 as usize] {
    Tile::BoxLeft => {
      if dir.1 != 0 {
        apply_move(field, (pos.0, pos.1 + dir.1), dir);
        apply_move(field, (pos.0 + 1, pos.1 + dir.1), dir);
      } else {
        apply_move(field, (pos.0 + 2, pos.1), dir);
      }
    }
    Tile::BoxRight => {
      if dir.1 != 0 {
        apply_move(field, (pos.0 - 1, pos.1 + dir.1), dir);
        apply_move(field, (pos.0, pos.1 + dir.1), dir);
      } else {
        apply_move(field, (pos.0 - 2, pos.1), dir);
      }
    }
    Tile::Air => return,
    Tile::Robot => apply_move(field, (pos.0 + dir.0, pos.1 + dir.1), dir),
    _ => unreachable!(),
  }

  match field[pos.1 as usize][pos.0 as usize] {
    Tile::BoxLeft => {
      if dir.1 != 0 {
        field[(pos.1 + dir.1) as usize][pos.0 as usize] = Tile::BoxLeft;
        field[(pos.1 + dir.1) as usize][(pos.0 + 1) as usize] = Tile::BoxRight;
        field[pos.1 as usize][pos.0 as usize] = Tile::Air;
        field[pos.1 as usize][(pos.0 + 1) as usize] = Tile::Air;
      } else {
        field[pos.1 as usize][(pos.0 + 2) as usize] = Tile::BoxRight;
        field[pos.1 as usize][(pos.0 + 1) as usize] = Tile::BoxLeft;
        field[pos.1 as usize][pos.0 as usize] = Tile::Air;
      }
    }
    Tile::BoxRight => {
      if dir.1 != 0 {
        field[(pos.1 + dir.1) as usize][pos.0 as usize] = Tile::BoxRight;
        field[(pos.1 + dir.1) as usize][(pos.0 - 1) as usize] = Tile::BoxLeft;
        field[pos.1 as usize][pos.0 as usize] = Tile::Air;
        field[pos.1 as usize][(pos.0 - 1) as usize] = Tile::Air;
      } else {
        field[pos.1 as usize][(pos.0 - 2) as usize] = Tile::BoxLeft;
        field[pos.1 as usize][(pos.0 - 1) as usize] = Tile::BoxRight;
        field[pos.1 as usize][pos.0 as usize] = Tile::Air;
      }
    }
    Tile::Robot => {
      field[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] =
        field[pos.1 as usize][pos.0 as usize];
      field[pos.1 as usize][pos.0 as usize] = Tile::Air;
    }
    _ => unreachable!(),
  }
}

fn movement2(field: &mut [Vec<Tile>], pos: &mut (isize, isize), dir: Direction) {
  let vec = dir.to_vector();

  if !is_moveable(field, *pos, vec) {
    return;
  }

  apply_move(field, *pos, vec);

  pos.0 += vec.0;
  pos.1 += vec.1;
}

fn double_wide(field: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
  field
    .into_iter()
    .map(|line| {
      line
        .into_iter()
        .flat_map(|tile| match tile {
          Tile::Air => vec![Tile::Air, Tile::Air],
          Tile::Wall => vec![Tile::Wall, Tile::Wall],
          Tile::Robot => vec![Tile::Robot, Tile::Air],
          Tile::Box => vec![Tile::BoxLeft, Tile::BoxRight],
          _ => unreachable!(),
        })
        .collect()
    })
    .collect()
}

fn get_gps(field: &[Vec<Tile>]) -> Vec<(usize, usize)> {
  let mut coords = Vec::new();

  for (y, line) in field.iter().enumerate() {
    for (x, t) in line.iter().enumerate() {
      if *t == Tile::Box || *t == Tile::BoxLeft {
        coords.push((x, y));
      }
    }
  }

  coords
}

fn movement(field: &mut [Vec<Tile>], pos: &mut (isize, isize), dir: Direction) {
  let vec = dir.to_vector();

  let mut current = *pos;
  loop {
    current.0 += vec.0;
    current.1 += vec.1;

    match field[current.1 as usize][current.0 as usize] {
      Tile::Wall => return,
      Tile::Air => break,
      _ => (),
    }
  }

  while current.0 != pos.0 || current.1 != pos.1 {
    let next = (current.0 - vec.0, current.1 - vec.1);
    field[current.1 as usize][current.0 as usize] = field[next.1 as usize][next.0 as usize];

    current = next;
  }

  field[pos.1 as usize][pos.0 as usize] = Tile::Air;
  pos.0 += vec.0;
  pos.1 += vec.1;
}

fn find_robot(field: &[Vec<Tile>]) -> (isize, isize) {
  for (y, line) in field.iter().enumerate() {
    for (x, t) in line.iter().enumerate() {
      if *t == Tile::Robot {
        return (x as isize, y as isize);
      }
    }
  }

  (0, 0)
}

fn parse(input: String) -> (Vec<Vec<Tile>>, Vec<Direction>) {
  let mut parts = input.split("\n\n");
  let field = parts
    .next()
    .unwrap()
    .split("\n")
    .map(|line| line.chars().flat_map(|c| format!("{c}").parse()).collect())
    .collect();

  let movements = parts
    .next()
    .unwrap()
    .split("\n")
    .flat_map(|line| line.chars().flat_map(|c| format!("{c}").parse()))
    .collect();

  (field, movements)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Tile {
  Wall,
  Box,
  Air,
  Robot,
  BoxLeft,
  BoxRight,
}

impl FromStr for Tile {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.chars().next() {
      Some('#') => Ok(Tile::Wall),
      Some('O') => Ok(Tile::Box),
      Some('.') => Ok(Tile::Air),
      Some('@') => Ok(Tile::Robot),
      _ => Err(()),
    }
  }
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl FromStr for Direction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.chars().next() {
      Some('<') => Ok(Direction::Left),
      Some('>') => Ok(Direction::Right),
      Some('^') => Ok(Direction::Up),
      Some('v') => Ok(Direction::Down),
      _ => Err(()),
    }
  }
}

impl Direction {
  fn to_vector(&self) -> (isize, isize) {
    match self {
      Self::Down => (0, 1),
      Self::Up => (0, -1),
      Self::Left => (-1, 0),
      Self::Right => (1, 0),
    }
  }
}
