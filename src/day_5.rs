use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let (rules, updates) = parse(input);

    let mut sum = 0;
    for update in &updates {
      if check_order(update, &rules) {
        sum += update[update.len() / 2];
      }
    }

    Ok(format!("{}", sum))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let (rules, updates) = parse(input);

    let wrong: Vec<Vec<usize>> = updates
      .into_iter()
      .filter(|update| !check_order(update, &rules))
      .collect();

    let sum: usize = wrong
      .into_iter()
      .map(|update| {
        let mut new = Vec::new();
        for num in update {
          insert_num(&mut new, &rules, num);
        }

        new[new.len() / 2]
      }).sum();

    Ok(format!("{}", sum))
  }
}

fn parse(input: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
  let mut parts = input.split("\n\n");
  let rules = parts.next().unwrap();
  let updates = parts.next().unwrap();

  let rules: Vec<(usize, usize)> = rules
    .split("\n")
    .map(|r| {
      let mut nums = r.split("|");
      let left: usize = nums.next().unwrap().parse().unwrap();
      let right: usize = nums.next().unwrap().parse().unwrap();
      (left, right)
    })
    .collect();

  let updates: Vec<Vec<usize>> = updates
    .split("\n")
    .map(|u| u.split(",").flat_map(|n| n.parse::<usize>()).collect())
    .collect();

  (rules, updates)
}

fn check_order(update: &[usize], rules: &[(usize, usize)]) -> bool {
  update
    .iter()
    .enumerate()
    .map(|(i, &num)| {
      rules
        .iter()
        .filter(|(l, r)| *l == num || *r == num)
        .map(|(l, r)| {
          if *l == num {
            let (before, after) = update.split_at(i + 1);
            after.contains(r) || !before.contains(r)
          } else {
            let (before, after) = update.split_at(i + 1);
            !after.contains(r) || before.contains(r)
          }
        })
        .all(|b| b)
    })
    .all(|b| b)
}

fn insert_num(update: &mut Vec<usize>, rules: &[(usize, usize)], num: usize) {
  if update.is_empty() {
    update.push(num);
    return;
  }

  let relevant: Vec<(usize, usize)> = rules
    .iter()
    .filter(|(l, r)| (*l == num && update.contains(r)) || (*r == num && update.contains(l)))
    .copied()
    .collect();

  let mut before = Vec::new();
  let mut after = Vec::new();

  for (l, r) in relevant {
    if l == num {
      after.push(r);
    } else {
      before.push(l);
    }
  }

  let mut index = 0;
  if !before.is_empty() {
    for (i, num) in update.iter().enumerate() {
      before.retain(|b| b != num);
      if before.is_empty() {
        index = i + 1;
        break;
      }
    }
  }

  let b = update
    .split_at_checked(index)
    .map(|u| u.1.iter().all(|a| after.contains(a)))
    .unwrap_or(true);

  if !b {
    panic!("Error");
  }

  update.insert(index, num);
}
