use std::ops::Range;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let robots = parse(input);
    let grid = (101, 103);

    let pos: Vec<(isize, isize)> = robots
      .into_iter()
      .map(|r| get_pos_after(r, grid, 100))
      .collect();
    let mut q_1 = 0;
    let mut q_2 = 0;
    let mut q_3 = 0;
    let mut q_4 = 0;

    for (x, y) in pos {
      if x < grid.0 / 2 && y < grid.1 / 2 {
        q_1 += 1;
      } else if x < grid.0 / 2 && y > (grid.1 / 2) {
        q_3 += 1;
      } else if y < grid.1 / 2 && x > (grid.0 / 2) {
        q_2 += 1;
      } else if x > (grid.0 / 2) && y > (grid.1 / 2) {
        q_4 += 1;
      }
    }

    Ok(format!("{}", q_1 * q_2 * q_3 * q_4))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let grid = (101, 103);
    let robots = parse(input);

    for i in 0..10000 {
      let pos: Vec<(isize, isize)> = robots
        .clone()
        .into_iter()
        .map(|r| get_pos_after(r, grid, i))
        .collect();

      if check_density(grid, &pos) {
        print_grid(&pos, grid);
        println!("{i}");
      }
    }

    Ok("".into())
  }
}

fn print_grid(robots: &[(isize, isize)], grid: (isize, isize)) {
  let mut field = Vec::new();
  for _ in 0..grid.1 {
    field.push(vec!['.'; grid.0 as usize]);
  }

  for (x, y) in robots {
    field[*y as usize][*x as usize] = '#';
  }

  for line in field {
    for char in line {
      print!("{char}");
    }
    println!();
  }
}

fn check_density(grid: (isize, isize), pos: &[(isize, isize)]) -> bool {
  let step_x = grid.0 / 10;
  let step_y = grid.1 / 10;
  for x in 0..step_x {
    for y in 0..step_y {
      let density = get_density(
        pos,
        x * step_x..x * step_x + step_x,
        y * step_y..y * step_y + step_y,
      );
      if density > 0.4 {
        return true;
      }
    }
  }

  false
}

fn get_density(robots: &[(isize, isize)], x: Range<isize>, y: Range<isize>) -> f32 {
  robots
    .iter()
    .filter(|(r_x, r_y)| x.contains(r_x) && y.contains(r_y))
    .count() as f32
    / (x.len() * y.len()) as f32
}

fn get_pos_after(
  robot: ((isize, isize), (isize, isize)),
  grid: (isize, isize),
  iter: isize,
) -> (isize, isize) {
  let mut x = (robot.0 .0 + robot.1 .0 * iter) % grid.0;
  let mut y = (robot.0 .1 + robot.1 .1 * iter) % grid.1;

  if x < 0 {
    x += grid.0;
  }
  if y < 0 {
    y += grid.1;
  }

  (x, y)
}

fn parse(input: String) -> Vec<((isize, isize), (isize, isize))> {
  input
    .split("\n")
    .map(|line| {
      let mut parts = line.split(" ");

      let pos = parse_x_y(parts.next().unwrap());
      let vel = parse_x_y(parts.next().unwrap());

      (pos, vel)
    })
    .collect()
}

fn parse_x_y(input: &str) -> (isize, isize) {
  let mut x_y = input.split("=").nth(1).unwrap().split(",");
  let x = x_y.next().unwrap().parse().unwrap();
  let y = x_y.next().unwrap().parse().unwrap();
  (x, y)
}
