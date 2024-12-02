#[derive(Debug, Clone)]
enum SafeListOrdering {
  Ascending,
  Descending,
}

#[derive(Debug, Clone)]
struct SafeList {
  ordering: SafeListOrdering,
  list: Vec<isize>,
  is_valid: bool,
}

impl SafeList {
  fn from(list: Vec<isize>) -> Self {
    let is_valid = list[0] != list[1];

    let ordering = if list[0] < list[1] {
      SafeListOrdering::Ascending
    } else {
      SafeListOrdering::Descending
    };

    SafeList {
      ordering,
      list,
      is_valid,
    }
  }

  fn check_validity(&mut self, index: usize) {
    let previous = self.list[index - 1];
    let current = self.list[index];

    let difference = current - previous;
    if difference.abs() > 3 {
      self.is_valid = false;
      return;
    }

    match self.ordering {
      SafeListOrdering::Ascending => {
        if current <= previous {
          self.is_valid = false;
        }
      }
      SafeListOrdering::Descending => {
        if current >= previous {
          self.is_valid = false;
        }
      }
    }
  }
}

pub fn part_one(input: &str) -> isize {
  let mut accumulator: isize = 0;

  for line in input.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    let values = line.split_whitespace().filter_map(|value| value.parse::<isize>().ok()).collect::<Vec<isize>>();

    let mut safe_list = SafeList::from(values);
    if !safe_list.is_valid {
      continue;
    }

    for value in 1..safe_list.list.len() {
      safe_list.check_validity(value);
      if !safe_list.is_valid {
        break;
      }
    }

    if safe_list.is_valid {
      accumulator += 1;
    }
  }
  accumulator
}

pub fn part_two(input: &str) -> isize {
  let mut accumulator: isize = 0;

  for line in input.lines() {
    let line = line.trim();
    if line.is_empty() {
      continue;
    }

    let values = line.split_whitespace().filter_map(|value| value.parse::<isize>().ok()).collect::<Vec<isize>>();

    let mut safe_list = SafeList::from(values);

    for value in 1..safe_list.list.len() {
      safe_list.check_validity(value);
      if !safe_list.is_valid {
        break;
      }
    }

    if safe_list.is_valid {
      accumulator += 1;
    } else {
      for index in 0..safe_list.list.len() {
        let mut list = safe_list.list.clone();
        list.remove(index);

        let mut new_list = SafeList::from(list);

        for value in 1..new_list.list.len() {
          new_list.check_validity(value);
          if !new_list.is_valid {
            break;
          }
        }

        if new_list.is_valid {
          accumulator += 1;
          break;
        }
      }
    }
  }
  accumulator
}


#[allow(dead_code)]
fn main() {
  let asset = include_str!("../assets/day-02/asset.txt");
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
    assert_eq!(part_two(example_assets), 4);
  }
}
