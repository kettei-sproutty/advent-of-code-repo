struct List {
  list: Vec<isize>
}

impl List {
  fn parse(input: &str) -> Self {
    List {
      list: input
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect::<Vec<isize>>(),
    }
  }

  fn check_valid(&self) -> bool {
    if self.list[0] == self.list[1] {
      return false;
    }

    let descending = self.list[0] < self.list[1];

    self
      .list
      .windows(2)
      .all(|window| {
        let is_correct_order = if descending {
          window[0] < window[1]
        } else {
          window[0] > window[1]
        };

        let has_in_range_difference = (window[0] - window[1]).abs() <= 3;

        is_correct_order && has_in_range_difference
      })
  }
}

pub fn part_one(input: &str) -> isize {
  let mut accumulator = 0;

  for line in input.lines() {
    if line.is_empty() {
      continue;
    }
    let list = List::parse(line);
    if list.check_valid() {
      accumulator += 1;
    }
  }

  accumulator
}

pub fn part_two(input: &str) -> isize {
  let mut accumulator = 0;

  for line in input.lines() {
    if line.is_empty() {
      continue;
    }

    let mut list = List::parse(line);

    for index in 0..list.list.len() {
      let deleted = list.list.remove(index);

      if list.check_valid() {
        accumulator += 1;
        break;
      }

      list.list.insert(index, deleted);
    }
  }

  accumulator
}

#[allow(dead_code)]
fn main() {
  let asset = include_str!("../assets/day-02/asset.txt");

  let part_1_result = part_one(asset);
  println!("Part 1 result: {part_1_result}");

  let part_2_result = part_two(asset);
  println!("Part 2 result: {part_2_result}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one_example() {
    let example_assets = include_str!("../assets/day-02/example.txt");
    assert_eq!(part_one(example_assets), 2);
  }

  #[test]
  fn test_part_two_example() {
    let example_assets = include_str!("../assets/day-02/example.txt");
    assert_eq!(part_two(example_assets), 4);
  }
}
