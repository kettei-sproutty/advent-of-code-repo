use std::collections::HashMap;

fn get_columns(input: &str) -> (Vec<isize>, Vec<isize>) {
  let mut left: Vec<isize> = vec![];
  let mut right: Vec<isize> = vec![];

  for line in input.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }
    let values = line.split(' ').collect::<Vec<&str>>();

    let left_value = values
      .get(0)
      .unwrap_or(&"0")
      .parse::<isize>()
      .expect("Cannot parse left to usize");

    let right_value = values
      .get(values.len() - 1)
      .unwrap_or(&"0")
      .parse::<isize>()
      .expect("Cannot parse right to usize");

    left.push(left_value);
    right.push(right_value);
  }

  (left, right)
}

pub fn part1(input: &str) -> isize {
  let (mut left, mut right) = get_columns(input);

  left.sort();
  right.sort();

  let mut accumulator: isize = 0;

  for index in 0..=left.len() - 1 {
    accumulator += (right[index] - left[index]).abs();
  }

  accumulator
}

pub fn part2(input: &str) -> isize {
  let (left, right) = get_columns(input);

  let mut accumulator: isize = 0;

  let mut right_map: HashMap<isize, isize> = HashMap::new();

  for value in right {
    let current = right_map.get(&value).unwrap_or(&0);
    right_map.insert(value, current + 1);
  }

  for index in 0..=left.len() - 1 {
    let value = left[index];
    let occurencies = right_map.get(&value).unwrap_or(&0);

    accumulator += value * occurencies;
  }

  accumulator
}

fn main() {
  let asset = include_str!("../assets/day-01/asset.txt");
  let part_1_result = part1(asset);

  println!("Part 1 result: {}", part_1_result);

  let part_2_result = part2(asset);

  println!("Part 2 result: {}", part_2_result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part1(example_assets), 11);
  }

  #[test]
  fn test_part2() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part2(example_assets), 31);
  }
}
