use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let tests = parse(input);

    let mut sum = 0;
    for (result, nums) in tests {
      if check_test(result, &nums, false) {
        sum += result;
      }
    }

    Ok(format!("{}", sum))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let tests = parse(input);

    let mut sum = 0;
    for (result, nums) in tests {
      if check_test(result, &nums, true) {
        sum += result;
      }
    }

    Ok(format!("{}", sum))
  }
}

fn check_test(result: usize, nums: &[usize], part_2: bool) -> bool {
  let combinations = generate_combinations(nums.len() - 1, part_2);

  for combination in combinations {
    let calculated = get_result(nums, &combination);
    if result == calculated {
      return true;
    }
  }

  false
}

fn get_result(nums: &[usize], operations: &[Operation]) -> usize {
  let mut current = nums[0];

  for i in 1..nums.len() {
    current = operations[i - 1].exec(current, nums[i]);
  }

  current
}

fn parse(input: String) -> Vec<(usize, Vec<usize>)> {
  input
    .split("\n")
    .map(|line| {
      let mut parts = line.split(": ");

      let result = parts.next().unwrap().parse().unwrap();
      let nums = parts
        .next()
        .unwrap()
        .split(" ")
        .flat_map(str::parse)
        .collect();

      (result, nums)
    })
    .collect()
}

#[derive(Clone, Debug)]
enum Operation {
  Add,
  Mul,
  Concat,
}

impl Operation {
  fn exec(&self, a: usize, b: usize) -> usize {
    match self {
      Operation::Add => a + b,
      Operation::Mul => a * b,
      Operation::Concat => format!("{}{}", a, b).parse().unwrap(),
    }
  }
}

fn generate_combinations(n: usize, part_2: bool) -> Vec<Vec<Operation>> {
  if n == 0 {
    return vec![vec![]];
  }

  let mut result = vec![];
  let smaller_combinations = generate_combinations(n - 1, part_2);

  for combination in smaller_combinations {
    let mut add_combination = combination.clone();
    add_combination.push(Operation::Add);
    result.push(add_combination);

    let mut mul_combination = combination.clone();
    mul_combination.push(Operation::Mul);
    result.push(mul_combination);

    if part_2 {
      let mut concat_combination = combination;
      concat_combination.push(Operation::Concat);
      result.push(concat_combination);
    }
  }

  result
}
