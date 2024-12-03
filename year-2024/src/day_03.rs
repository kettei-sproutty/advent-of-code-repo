pub use regex::Regex;

pub fn part_one(input: &str) -> isize {
 Regex::new(r"mul\((\d+,\d+)\)").unwrap().captures_iter(input).fold(0, |acc, cap| {
    let mut iter = cap[1].split(',');
    let a = iter.next().unwrap().parse::<isize>().unwrap();
    let b = iter.next().unwrap().parse::<isize>().unwrap();
    acc + a * b
  })
}

pub fn part_one_no_regex(input: &str) -> isize {
  let mut input = input;
  let mut accumulator = 0;

  loop {
    let mul_position = input.find("mul(");
    if mul_position.is_none() {
      break;
    }

    let mul_position = mul_position.unwrap();
    let start = mul_position + 4;

    let end = input[start..].find(')');

    if end.is_none() {
      break;
    }

    let end = end.unwrap();
    let numbers = &input[start..start + end];

    let mut iter = numbers.split(',');
    let a = iter.next().unwrap().parse::<isize>();
    if a.is_err() {
      input = &input[start..];
      continue;
    }
    let b = iter.next().unwrap().parse::<isize>();
    if b.is_err() {
      input = &input[start..];
      continue;
    }

    let a = a.unwrap();
    let b = b.unwrap();

    accumulator += a * b;

    input = &input[start..];
  }

  accumulator
}

pub fn part_two(input: &str) -> isize {
  let mut is_dont = false;
  Regex::new(r"mul\((\d+,\d+)\)|do\(\)|don't\(\)").unwrap().captures_iter(input).fold(0, |acc, cap| {
    let is_modifier = cap.get(1).is_none();

    if is_modifier{
      is_dont = cap.get(0).unwrap().as_str().contains("don't");
      return acc;
    }

    if is_dont {
      return acc;
    }

    let mut iter = cap[1].split(',');
    let a = iter.next().unwrap().parse::<isize>().unwrap();
    let b = iter.next().unwrap().parse::<isize>().unwrap();
    acc + a * b
  })
}

const DO_SIZE: usize = "do()".len();
const MUL_SIZE: usize = "mul(".len();

pub fn part_two_no_regex(input: &str) -> isize {
  let mut input = input;
  let mut accumulator = 0;

  let parse = |input: &str| -> Option<(isize, isize)> {
    let mut iter = input.split(',');
    let a = iter.next();
    if a.is_none() {
      return None
    }

    let b = iter.next();
    if b.is_none() {
      return None
    }


    let a = a.unwrap().parse::<isize>();
    let b = b.unwrap().parse::<isize>();

    if a.is_err() || b.is_err() {
      return None
    }

    let a = a.unwrap();
    let b = b.unwrap();

    Some((a, b))
  };

  loop {
    let dont_position = input.find("don't(");
    let do_position = input.find("do(");
    let mul_position = input.find("mul(");

    if dont_position.is_none() && do_position.is_none() && mul_position.is_none() {
      break;
    }

    let dont_position = dont_position.unwrap_or(usize::MAX);
    let do_position = do_position.unwrap_or(usize::MAX);
    let mul_position = mul_position.unwrap_or(usize::MAX);

    if dont_position < do_position && dont_position < mul_position {
      if do_position < mul_position {
        let start = mul_position + MUL_SIZE;
        let end = input[start..].find(')').unwrap();
        let numbers = &input[start..start + end];

        let numbers = parse(numbers);
        if numbers.is_none() {
          input = &input[start..];
          continue;
        }

        let (a, b) = numbers.unwrap();

        accumulator += a * b;

        input = &input[start + end..];
        continue;
      }

      if do_position == usize::MAX {
        break;
      }

      input = &input[do_position + DO_SIZE..];
      continue;
    }

    if do_position < dont_position && do_position < mul_position {
      let start = mul_position + MUL_SIZE;
      let end = input[start..].find(')').unwrap();
      let numbers = &input[start..start + end];

      let numbers = parse(numbers);
      if numbers.is_none() {
        input = &input[start..];
        continue;
      }

      let (a, b) = numbers.unwrap();

      accumulator += a * b;

      input = &input[start + end..];
    }

    if mul_position < dont_position && mul_position < do_position {
      let start = mul_position + MUL_SIZE;
      let end = input[start..].find(')').unwrap();
      let numbers = &input[start..start + end];

      let numbers = parse(numbers);
      if numbers.is_none() {
        input = &input[start..];
        continue;
      }

      let (a, b) = numbers.unwrap();

      accumulator += a * b;

      input = &input[start + end..];

      continue;
    }
  }



  accumulator
}

pub fn part_two_o_n(input: &str) -> isize {
  let mut accumulator = 0;
  let mut is_dont = false;
  let mut action_checker = String::new();

  for char in input.chars() {
    action_checker.push(char);
    if action_checker.ends_with("don't()") {
      is_dont = true;
      action_checker.clear();
      continue;
    }

    if action_checker.ends_with("do()") {
      is_dont = false;
      action_checker.clear();
      continue;
    }

    if action_checker.ends_with("mul(") && !is_dont {
      action_checker = "mul(".to_string();
      continue;
    }

    if action_checker.starts_with("mul(") && char == ')' {
      if is_dont {
        action_checker.clear();
        continue;
      }
      action_checker.pop();
      action_checker = action_checker[MUL_SIZE..].to_string();
      let mut iter = action_checker.split(',');
      let a = iter.next();

      if a.is_none() {
        action_checker.clear();
        continue;
      }

      let b = iter.next();
      if b.is_none() {
        action_checker.clear();
        continue;
      }

      let a = a.unwrap().parse::<isize>();
      if a.is_err() {
        action_checker.clear();
        continue;
      }

      let b = b.unwrap().parse::<isize>();
      if b.is_err() {
        action_checker.clear();
        continue;
      }

      let a = a.unwrap();
      let b = b.unwrap();

      accumulator += a * b;
    }
  }

  accumulator
}

#[allow(dead_code)]
fn main() {
  let asset = include_str!("../assets/day-03/asset.txt");

  let part_1_result = part_one(asset);
  println!("Part 1 result: {part_1_result}");

  let part_1_no_regex_result = part_one_no_regex(asset);
  println!("Part 1 no regex result: {part_1_no_regex_result}");

  let part_2_result = part_two(asset);
  println!("Part 2 result: {part_2_result}");

  let part_2_no_regex_result = part_two_no_regex(asset);
  println!("Part 2 no regex result: {part_2_no_regex_result}");

  let part_2_o_n_result = part_two_o_n(asset);
  println!("Part 2 O(n) result: {part_2_o_n_result}");
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
  fn test_part_one_no_regex_example() {
    let example_assets = include_str!("../assets/day-03/example.txt");
    assert_eq!(part_one_no_regex(example_assets), 161);
  }

  #[test]
  fn test_part_two_example() {
    let example_assets = include_str!("../assets/day-03/example.txt");
    assert_eq!(part_two(example_assets), 48);
  }

  #[test]
  fn test_part_two_no_regex_example() {
    let example_assets = include_str!("../assets/day-03/example.txt");
    assert_eq!(part_two_no_regex(example_assets), 48);
  }

  #[test]
  fn test_part_two_o_n_example() {
    let example_assets = include_str!("../assets/day-03/example.txt");
    assert_eq!(part_two_o_n(example_assets), 48);
  }
}
