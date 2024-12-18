use std::collections::VecDeque;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let coords = parse(input);

    let mut field = Vec::new();
    for _ in 0..71 {
      field.push(vec![Tile::Safe; 71]);
    }

    for (x, y) in &coords[0..1024] {
      field[*y][*x] = Tile::Unsafe;
    }

    let cost = find_path(&field);

    Ok(format!("{}", cost))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let coords = parse(input);

    let mut field = Vec::new();
    for _ in 0..71 {
      field.push(vec![Tile::Safe; 71]);
    }

    let mut byte = (0, 0);
    for (x, y) in &coords {
      field[*y][*x] = Tile::Unsafe;
      let cost = find_path(&field);
      if cost == 0 {
        byte = (*x, *y);
        break;
      }
    }

    Ok(format!("{},{}", byte.0, byte.1))
  }
}

fn parse(input: String) -> Vec<(usize, usize)> {
  input
    .split("\n")
    .map(|line| {
      let mut parts = line.split(",");
      let x = parts.next().unwrap().parse().unwrap();
      let y = parts.next().unwrap().parse().unwrap();
      (x, y)
    })
    .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
  Safe,
  Unsafe,
}

fn find_path(field: &[Vec<Tile>]) -> usize {
  let mut visited = Vec::new();
  let mut queue = VecDeque::new();
  queue.push_back((0_isize, 0_isize, 0));

  while let Some((x, y, cost)) = queue.pop_front() {
    for (dif_x, dif_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
      if let Some(line) = field.get((y + dif_y) as usize) {
        if let Some(tile) = line.get((x + dif_x) as usize) {
          let pos = (x + dif_x, y + dif_y);
          if pos == (70, 70) {
            return cost + 1;
          } else if *tile == Tile::Safe && !visited.contains(&pos) {
            queue.push_back((pos.0, pos.1, cost + 1));
            visited.push(pos);
          }
        }
      }
    }
  }

  0
}
