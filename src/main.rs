use anyhow::Error;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
  let args = std::env::args().collect::<Vec<String>>();
  if args.len() != 2 {
    println!("Usage: ./aoc_2024 <day>");
    std::process::exit(0);
  }

  let Ok(day) = args[1].parse::<u8>() else {
    println!("Usage: ./aoc_2024 <day>");
    std::process::exit(0);
  };

  if !(1..=25).contains(&day) {
    println!("Day must be between 1 and 25");
    std::process::exit(0);
  }

  let Ok(input_raw) = std::fs::read(format!("input/day_{}", day)) else {
    println!("Could not read input file input/day_{}", day);
    std::process::exit(0);
  };
  let Ok(input) = String::from_utf8(input_raw) else {
    println!("Could not parse input file input/day_{} to UTF-8", day);
    std::process::exit(0);
  };

  let answer: &dyn Solution = match day {
    1 => &day_1::Answer,
    2 => &day_2::Answer,
    3 => &day_3::Answer,
    4 => &day_4::Answer,
    5 => &day_5::Answer,
    6 => &day_6::Answer,
    7 => &day_7::Answer,
    8 => &day_8::Answer,
    9 => &day_9::Answer,
    10 => &day_10::Answer,
    11 => &day_11::Answer,
    12 => &day_12::Answer,
    13 => &day_13::Answer,
    14 => &day_14::Answer,
    15 => &day_15::Answer,
    16 => &day_16::Answer,
    17 => &day_17::Answer,
    18 => &day_18::Answer,
    19 => &day_19::Answer,
    20 => &day_20::Answer,
    21 => &day_21::Answer,
    22 => &day_22::Answer,
    23 => &day_23::Answer,
    24 => &day_24::Answer,
    25 => &day_25::Answer,
    _ => unreachable!(),
  };

  let solution1 = match answer.part1(input.clone()) {
    Ok(s) => s,
    Err(err) => {
      println!("Error while calculating part 1: {:?}", err);
      std::process::exit(0);
    }
  };
  let solution2 = match answer.part2(input) {
    Ok(s) => s,
    Err(err) => {
      println!("Error while calculating part 2: {:?}", err);
      std::process::exit(0);
    }
  };

  println!("Solution part 1 is: {}", solution1);
  println!("Solution part 2 is: {}", solution2);
}

trait Solution {
  fn part1(&self, input: String) -> Result<String, Error>;
  fn part2(&self, input: String) -> Result<String, Error>;
}
