use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let prices = parse(input);
    let sums: i64 = prices.into_iter().flat_map(|p| calc(p.a, p.b, p.pos)).sum();
    Ok(format!("{}", sums))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let prices = parse(input);
    let sums: i64 = prices
      .into_iter()
      .map(|mut p| {
        p.pos.0 += 10_000_000_000_000;
        p.pos.1 += 10_000_000_000_000;
        p
      })
      .flat_map(|p| calc(p.a, p.b, p.pos))
      .sum();
    Ok(format!("{}", sums))
  }
}

fn calc(a: (i64, i64), b: (i64, i64), pos: (i64, i64)) -> Option<i64> {
  let det = a.0 * b.1 - a.1 * b.0;

  if det == 0 {
    return None;
  }

  let b_num = a.0 * pos.1 - a.1 * pos.0;
  let a_num = b.1 * pos.0 - b.0 * pos.1;

  if b_num % det == 0 && a_num % det == 0 {
    let b_val = b_num / det;
    let a_val = a_num / det;
    if b_val >= 0 && a_val >= 0 {
      Some(b_val + a_val * 3)
    } else {
      None
    }
  } else {
    None
  }
}

fn parse(input: String) -> Vec<Price> {
  input
    .split("\n\n")
    .map(|price| {
      let mut parts = price.split("\n");
      let a = parse_button(parts.next().unwrap(), "+");
      let b = parse_button(parts.next().unwrap(), "+");
      let pos = parse_button(parts.next().unwrap(), "=");

      Price { pos, a, b }
    })
    .collect()
}

#[derive(Debug)]
struct Price {
  pos: (i64, i64),
  a: (i64, i64),
  b: (i64, i64),
}

fn parse_button(input: &str, split: &str) -> (i64, i64) {
  let mut parts = input.split(split);
  let x: i64 = parts
    .nth(1)
    .unwrap()
    .split(",")
    .next()
    .unwrap()
    .parse()
    .unwrap();
  let y: i64 = parts.next().unwrap().parse().unwrap();

  (x, y)
}
