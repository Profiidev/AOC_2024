use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let area = parse(input);

    let mut sum = 0;
    let mut used = HashSet::new();
    for (y, line) in area.iter().enumerate() {
      for (x, field) in line.iter().enumerate() {
        if used.contains(&(x as isize, y as isize)) {
          continue;
        }
        let plot = grow_plot(&area, x as isize, y as isize, *field, &mut used);
        sum += plot.len() * get_perimeter(&plot);
      }
    }
    Ok(format!("{}", sum))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let area = parse(input);

    let mut sum = 0;
    let mut used = HashSet::new();
    for (y, line) in area.iter().enumerate() {
      for (x, field) in line.iter().enumerate() {
        if used.contains(&(x as isize, y as isize)) {
          continue;
        }
        let plot = grow_plot(&area, x as isize, y as isize, *field, &mut used);
        sum += plot.len() * get_sides(&plot);
      }
    }
    Ok(format!("{}", sum))
  }
}

fn parse(input: String) -> Vec<Vec<char>> {
  input
    .split("\n")
    .map(|line| line.chars().collect())
    .collect()
}

fn get_sides(plot: &HashSet<(isize, isize)>) -> usize {
  let mut sides_y: HashMap<(isize, i32), Vec<isize>> = HashMap::new();
  let mut sides_x: HashMap<(isize, i32), Vec<isize>> = HashMap::new();
  for (x, y) in plot {
    if !plot.contains(&(*x + 1, *y)) {
      sides_y.entry((*x, 1)).or_default().push(*y);
    }

    if !plot.contains(&(*x, *y + 1)) {
      sides_x.entry((*y, 1)).or_default().push(*x);
    }

    if !plot.contains(&(*x - 1, *y)) {
      sides_y.entry((*x, -1)).or_default().push(*y);
    }

    if !plot.contains(&(*x, *y - 1)) {
      sides_x.entry((*y, -1)).or_default().push(*x);
    }
  }

  let mut sum = 0;
  for (_, ys) in sides_y {
    let mut done = HashSet::new();
    for y in &ys {
      if !done.contains(y) {
        test_side(&ys, *y, &mut done);
        sum += 1;
      }
    }
  }
  for (_, ys) in sides_x {
    let mut done = HashSet::new();
    for y in &ys {
      if !done.contains(y) {
        test_side(&ys, *y, &mut done);
        sum += 1;
      }
    }
  }

  sum
}

fn test_side(sides: &[isize], y: isize, done: &mut HashSet<isize>) {
  done.insert(y);
  if sides.contains(&(y + 1)) && !done.contains(&(y + 1)) {
    test_side(sides, y + 1, done);
  }

  if sides.contains(&(y - 1)) && !done.contains(&(y - 1)) {
    test_side(sides, y - 1, done);
  }
}

fn get_perimeter(plot: &HashSet<(isize, isize)>) -> usize {
  let mut sum = 0;
  for (x, y) in plot {
    if !plot.contains(&(*x + 1, *y)) {
      sum += 1;
    }

    if !plot.contains(&(*x, *y + 1)) {
      sum += 1;
    }

    if !plot.contains(&(*x - 1, *y)) {
      sum += 1;
    }

    if !plot.contains(&(*x, *y - 1)) {
      sum += 1;
    }
  }

  sum
}

fn grow_plot(
  area: &[Vec<char>],
  x: isize,
  y: isize,
  field: char,
  used: &mut HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
  let mut queue = VecDeque::new();
  queue.push_back((x, y));
  let mut plot = HashSet::new();
  plot.insert((x, y));

  while let Some((x, y)) = queue.pop_front() {
    let next = (x + 1, y);
    if let Some(c) = get_field(area, next) {
      if c == field && !used.contains(&next) {
        queue.push_back(next);
        used.insert(next);
        plot.insert(next);
      }
    }

    let next = (x, y + 1);
    if let Some(c) = get_field(area, next) {
      if c == field && !used.contains(&next) {
        queue.push_back(next);
        used.insert(next);
        plot.insert(next);
      }
    }

    let next = (x - 1, y);
    if let Some(c) = get_field(area, next) {
      if c == field && !used.contains(&next) {
        queue.push_back(next);
        used.insert(next);
        plot.insert(next);
      }
    }

    let next = (x, y - 1);
    if let Some(c) = get_field(area, next) {
      if c == field && !used.contains(&next) {
        queue.push_back(next);
        used.insert(next);
        plot.insert(next);
      }
    }
  }

  plot
}

fn get_field(area: &[Vec<char>], pos: (isize, isize)) -> Option<char> {
  if pos.0 < 0 || pos.1 < 0 {
    return None;
  }
  let pos = (pos.0 as usize, pos.1 as usize);

  if let Some(line) = area.get(pos.1) {
    line.get(pos.0).copied()
  } else {
    None
  }
}
