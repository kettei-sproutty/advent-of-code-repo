pub fn part_one(input: &str) -> isize {
  0
}

pub fn part_two(input: &str) -> isize {
  0
}

#[allow(dead_code)]
fn main() {
  let asset = include_str!("../assets/day-{{day}}/asset.txt");
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
    let example_assets = include_str!("../assets/day-{{day}}/example.txt");
    assert_eq!(part_one(example_assets), 0);
  }

  #[test]
  fn test_part_two_example() {
    let example_assets = include_str!("../assets/day-{{day}}/example.txt");
    assert_eq!(part_two(example_assets), 0);
  }
}
