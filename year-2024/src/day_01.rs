use std::collections::HashMap;

fn get_columns(input: &str) -> (Vec<isize>, Vec<isize>) {
  let mut left: Vec<isize> = vec![];
  let mut right: Vec<isize> = vec![];

  for line in input.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    let mut values = line.split_whitespace();

    let left_value = values.next().unwrap()
      .parse::<isize>()
      .expect("Cannot parse left to isize");

    let right_value = values.next().unwrap()
      .parse::<isize>()
      .expect("Cannot parse right to isize");


    left.push(left_value);
    right.push(right_value);
  }

  (left, right)
}

pub fn part_one(input: &str) -> isize {
  let (mut left, mut right) = get_columns(input);

  left.sort_unstable();
  right.sort_unstable();

  let mut accumulator: isize = 0;

  for index in 0..left.len() {
    accumulator += (right[index] - left[index]).abs();
  }

  accumulator
}

pub fn part_one_zip(input: &str) -> isize {
  let (mut left, mut right) = get_columns(input);

  left.sort_unstable();
  right.sort_unstable();

  left.iter().zip(right.iter())
    .map(|(left, right)| (right - left).abs())
    .sum()
}

pub fn part_one_insert_sort(input: &str) -> isize {
  let size = input.lines().count();
  let mut left = Vec::with_capacity(size);
  let mut right = Vec::with_capacity(size);

 for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    let mut left_insert_position = 0;
    let mut right_insert_position = 0;

    let mut values = line.split_whitespace();

    let left_value = values.next().unwrap()
      .parse::<isize>()
      .expect("Cannot parse left to isize");

    let right_value = values.last().unwrap()
      .parse::<isize>()
      .expect("Cannot parse right to isize");

    while left_insert_position < left.len() && left[left_insert_position] < left_value {
      left_insert_position += 1;
    }

    while right_insert_position < right.len() && right[right_insert_position] < right_value {
      right_insert_position += 1;
    }

    left.insert(left_insert_position, left_value);
    right.insert(right_insert_position, right_value);
  }

   left.iter().zip(right.iter())
    .map(|(left, right)| (right - left).abs())
    .sum()
}

pub fn part_two(input: &str) -> isize {
  let (left, right) = get_columns(input);

  let mut accumulator: isize = 0;

  let mut right_map: HashMap<isize, isize> = HashMap::new();

  for value in right {
    let current = right_map.get(&value).unwrap_or(&0);
    right_map.insert(value, current + 1);
  }

  for value in left {
    let occurencies = right_map.get(&value).unwrap_or(&0);

    accumulator += value * occurencies;
  }

  accumulator
}

pub fn part_two_directly_parsed(input: &str) -> isize {
  let mut accumulator = 0;
  let mut left: Vec<isize> = vec![];
  let mut right = HashMap::new();

  for line in input.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    let mut values = line.split_whitespace();

    let left_value = values.next().unwrap()
      .parse::<isize>()
      .expect("Cannot parse left to isize");

    let right_value = values.next().unwrap()
      .parse::<isize>()
      .expect("Cannot parse right to isize");

    left.push(left_value);

    let current = right.get(&right_value).unwrap_or(&0);
    right.insert(right_value, current + 1);
  }

  for value in left {
    let occurencies = right.get(&value).unwrap_or(&0);

    accumulator += value * occurencies;
  }

  accumulator
}

#[allow(dead_code)]
fn main() {
  let asset = include_str!("../assets/day-01/asset.txt");

  let part_1_result = part_one(asset);
  println!("Part 1 result: {part_1_result}");

  let part_1_zip_result = part_one_zip(asset);
  println!("Part 1 zip result: {part_1_zip_result}");

  let part_1_insert_sort_result = part_one_insert_sort(asset);
  println!("Part 1 dynamic sort result: {part_1_insert_sort_result}");

  let part_2_result = part_two(asset);
  println!("Part 2 result: {part_2_result}");

  let part_2_directly_parsed_result = part_two_directly_parsed(asset);
  println!("Part 2 directly parsed result: {part_2_directly_parsed_result}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one_example() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part_one(example_assets), 11);
  }

  #[test]
  fn test_part_one_zip_example() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part_one_zip(example_assets), 11);
  }

  #[test]
  fn test_part_one_insert_sort_example() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part_one_insert_sort(example_assets), 11);
  }

  #[test]
  fn test_part_two_example() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part_two(example_assets), 31);
  }

  #[test]
  fn test_part_two_directly_parsed_example() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part_two_directly_parsed(example_assets), 31);
  }
}
