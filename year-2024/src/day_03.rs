pub use regex::Regex;

pub fn part_one(input: &str) -> isize {
 Regex::new(r"mul\((\d+,\d+)\)").unwrap().captures_iter(input).fold(0, |acc, cap| {
    let mut iter = cap[1].split(",");
    let a = iter.next().unwrap().parse::<isize>().unwrap();
    let b = iter.next().unwrap().parse::<isize>().unwrap();
    acc + a * b
  })
}

pub fn part_two(_input: &str) -> isize {
  let mut is_dont = false;
  Regex::new(r"mul\((\d+,\d+)\)|do\(\)|don't\(\)").unwrap().captures_iter(_input).fold(0, |acc, cap| {
    let is_modifier = cap.get(1).is_none();

    if is_modifier{
      is_dont = cap.get(0).unwrap().as_str().contains("don't");
      return acc;
    }

    if is_dont {
      return acc;
    }

    let mut iter = cap[1].split(",");
    let a = iter.next().unwrap().parse::<isize>().unwrap();
    let b = iter.next().unwrap().parse::<isize>().unwrap();
    acc + a * b
  })
}

#[allow(dead_code)]
fn main() {
  let asset = include_str!("../assets/day-03/asset.txt");
  let part_1_result = part_one(asset);

  println!("Part 1 result: {}", part_1_result);

  let part_2_result = part_two(asset);

  println!("Part 2 result: {}", part_2_result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one_example() {
    let example_assets = include_str!("../assets/day-03/example.txt");
    assert_eq!(part_one(example_assets), 161);
  }

  #[test]
  fn test_part_two_example() {
    let example_assets = include_str!("../assets/day-03/example.txt");
    assert_eq!(part_two(example_assets), 48);
  }
}
