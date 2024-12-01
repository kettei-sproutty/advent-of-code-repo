pub fn part1(input: &str) -> isize {
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
      .expect("Cannot convert to usize");

    let right_value = values
      .get(values.len() - 1)
      .unwrap_or(&"0")
      .parse::<isize>()
      .expect("Cannot parse right to usize");

    left.push(left_value);
    right.push(right_value);
  }

  left.sort();
  right.sort();

  let mut accumulator: isize = 0;

  for index in 0..=left.len() - 1 {
    accumulator += (right[index] - left[index]).abs();
  }

  accumulator
}

fn main() {
  let part_1 = include_str!("../assets/day-01/part-1.txt");
  let part_1_result = part1(part_1);

  println!("Part 1 result: {}", part_1_result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let example_assets = include_str!("../assets/day-01/example.txt");
    assert_eq!(part1(example_assets), 11);
  }
}
