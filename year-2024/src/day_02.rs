#[derive(Debug)]
enum Sequence {
  Increasing,
  Decreasing,
}

pub fn part_one(input: &str) -> isize {
  let mut accumulator: isize = 0;
  for line in input.lines() {
    if line.is_empty() {
      continue;
    }
    let elements = line
      .split_whitespace()
      .into_iter()
      .filter_map(|x| x.parse::<isize>().ok())
      .collect::<Vec<isize>>();

    let mut is_safe = true;
    let mut sequence: Option<Sequence> = None;

    for (index, element) in elements.iter().enumerate() {
      if index == 0 {
        continue;
      }

      let previous = elements.get(index - 1).unwrap();
      if sequence.is_none() {
        if element > previous {
          sequence = Some(Sequence::Increasing);
        } else if element < previous {
          sequence = Some(Sequence::Decreasing);
        } else {
          is_safe = false;
          break;
        }
      }

      let difference = element - previous;
      if difference.abs() > 3 || difference.abs() == 0 {
        is_safe = false;
        break;
      }

      if let Some(Sequence::Increasing) = sequence {
        if difference < 0 {
          is_safe = false;
          break;
        }
      }

      if let Some(Sequence::Decreasing) = sequence {
        if difference > 0 {
          is_safe = false;
          break;
        }
      }
    }

    if is_safe {
      accumulator += 1;
    }
  }

  accumulator
}

pub fn part_two(_input: &str) -> isize {
  0
}

#[allow(dead_code)]
fn main() {
  let asset = include_str!("../assets/day-02/asset-part-one.txt");
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
    let example_assets = include_str!("../assets/day-02/example.txt");
    assert_eq!(part_one(example_assets), 2);
  }

  #[test]
  fn test_part_two_example() {
    let example_assets = include_str!("../assets/day-02/example.txt");
    assert_eq!(part_two(example_assets), 0);
  }
}
