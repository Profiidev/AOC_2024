use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let (mut pc, _) = parse(input).unwrap();
    let out: Vec<String> = pc.run(None).into_iter().map(|n| n.to_string()).collect();
    Ok(out.join(","))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let (pc, codes) = parse(input).unwrap();
    let res = solve_2(pc, 1, 16, codes.clone()).into_iter().min();

    Ok(format!("{}", res.unwrap()))
  }
}

fn solve_2(pc: PC, batch_size: usize, max: usize, codes: Vec<usize>) -> Vec<usize> {
  let mut possible = vec![0];

  for batch in 0..max / batch_size {
    let offset = batch * 3 * batch_size;
    let mask = mask(offset + batch_size * 3);
    let possible_last = possible.clone();
    possible.clear();

    for i in 1 << ((batch_size - 1) * 3)..1 << ((batch_size * 3) + 7).min(48 - offset) {
      for p in &possible_last {
        let mut pc = pc;
        let x = (i << offset) + p;
        pc.0 = x;

        let out = pc.run(None);

        if codes[0..(offset / 3 + batch_size)] == out[0..(offset / 3 + batch_size)]
          && !possible.iter().any(|p| x & mask == *p)
        {
          possible.push(x & mask);
        }
      }
    }
  }

  possible
}

fn mask(i: usize) -> usize {
  let mut res = 1;
  for _ in 0..i - 1 {
    res <<= 1;
    res += 1;
  }
  res
}

fn parse(input: String) -> Option<(PC, Vec<usize>)> {
  let mut parts = input.split("\n\n");
  let mut reg_parts = parts.next().unwrap().split("\n");

  let a = reg_parts.next()?.split(" ").last()?.parse().ok()?;

  let inst_parts = parts.next().unwrap().split(" ");
  let codes: Vec<usize> = inst_parts
    .last()
    .unwrap()
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  Some((PC(a), codes))
}

#[derive(Debug, Clone, Copy)]
struct PC(usize);

impl PC {
  fn run(&mut self, out: Option<&[usize]>) -> Vec<usize> {
    let mut output = Vec::new();
    loop {
      let b = (self.0 % 8) ^ 2;
      let b = ((b ^ 7) ^ (self.0 >> b)) % 8;
      self.0 /= 8;

      if let Some(out) = out {
        if out[output.len()] != b {
          return vec![];
        }
      }
      output.push(b);

      if self.0 == 0 {
        break;
      }
    }
    output
  }
}
