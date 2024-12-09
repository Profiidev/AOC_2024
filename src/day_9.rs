use std::cmp::Ordering;

use anyhow::Error;

use crate::Solution;

#[derive(Default)]
pub struct Answer;

impl Solution for Answer {
  fn part1(&self, input: String) -> Result<String, Error> {
    let mut fs = parse(input);

    let mut i = 0;
    let mut j = fs.len() - 1;
    while i <= j {
      if fs[i].is_some() {
        i += 1;
      } else if fs[j].is_some() {
        fs[i] = fs[j];
        fs[j] = None;
      } else {
        j -= 1;
      }
    }

    let mut sum = 0;
    for (i, num) in fs.into_iter().enumerate() {
      if let Some(num) = num {
        sum += i * num;
      }
    }

    Ok(format!("{}", sum))
  }

  fn part2(&self, input: String) -> Result<String, Error> {
    let mut fs = parse2(input);

    let mut j = fs.len() - 1 - ((fs.len() - 1) % 2);
    loop {
      let (file_index, file_size) = *fs.get(j).unwrap();

      for (i, (index, size)) in fs.clone().into_iter().enumerate() {
        if index.is_none() && i < j {
          match size.cmp(&file_size) {
            Ordering::Equal => {
              fs[i] = (file_index, file_size);
              insert_empty(&mut fs, j, file_size);
              break;
            }
            Ordering::Greater => {
              fs[i] = (file_index, file_size);
              insert_empty(&mut fs, j, file_size);
              fs.insert(i + 1, (None, size - file_size));
              break;
            }
            _ => (),
          }
        }
      }

      if j == 0 {
        break;
      }
      j = find_next_file(&fs, file_index.unwrap());
    }

    let mut sum = 0;
    let mut i = 0;
    for (index, size) in fs.into_iter() {
      if let Some(index) = index {
        sum += (size * i + (size * (size - 1)) / 2) * index;
      }
      i += size;
    }

    Ok(format!("{}", sum))
  }
}

fn parse(input: String) -> Vec<Option<usize>> {
  let mut fs = Vec::new();

  for (i, c) in input.chars().enumerate() {
    let size: usize = format!("{}", c).parse().unwrap();
    let index = if i % 2 == 0 { Some(i / 2) } else { None };

    for _ in 0..size {
      fs.push(index);
    }
  }

  fs
}

fn parse2(input: String) -> Vec<(Option<usize>, usize)> {
  let mut fs = Vec::new();

  for (i, c) in input.chars().enumerate() {
    let size: usize = format!("{}", c).parse().unwrap();
    let index = if i % 2 == 0 { Some(i / 2) } else { None };

    fs.push((index, size));
  }

  fs
}

fn insert_empty(fs: &mut Vec<(Option<usize>, usize)>, pos: usize, size: usize) {
  if let Some((None, _)) = fs.get(pos - 1).copied() {
    fs[pos - 1].1 += size;
    if let Some((None, size)) = fs.get(pos + 1).copied() {
      fs[pos - 1].1 += size;
      fs.remove(pos + 1);
    }
    fs.remove(pos);
  } else if let Some((None, next_size)) = fs.get(pos + 1).copied() {
    fs[pos] = (None, size + next_size);
    fs.remove(pos + 1);
  } else {
    fs[pos] = (None, size);
  }
}

fn find_next_file(fs: &[(Option<usize>, usize)], last: usize) -> usize {
  for (i, (index, _)) in fs.iter().enumerate().rev() {
    if let Some(index) = index {
      if *index == last - 1 {
        return i;
      }
    }
  }

  panic!("Error")
}
