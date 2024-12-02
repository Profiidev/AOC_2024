use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let reports = parse(input);

    let safe = reports.into_iter().filter(|report| is_save(report)).count();

    Ok(format!("{}", safe))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let reports = parse(input);

    let safe = reports
      .into_iter()
      .filter(|report| {
        (0..report.len())
          .map(|i| {
            let mut report = report.clone();
            report.remove(i);
            report
          })
          .any(|report| is_save(&report))
      })
      .count();

    Ok(format!("{}", safe))
  }
}

fn is_save(report: &[usize]) -> bool {
  let mut diffs = Vec::new();
  for i in 0..report.len() - 1 {
    diffs.push(report[i] as isize - report[i + 1] as isize);
  }

  diffs.iter().all(|num| (1..=3).contains(num)) || diffs.iter().all(|num| (-3..=-1).contains(num))
}

fn parse(input: String) -> Vec<Vec<usize>> {
  input
    .split("\n")
    .map(|line| {
      line
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
    })
    .collect()
}
