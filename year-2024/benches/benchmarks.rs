use year_2024::*;

fn main() {
  divan::main();
}

#[divan::bench]
fn day_01_part_one() {
  day_01::part_one(divan::black_box(include_str!("../assets/day-01/asset.txt")));
}

#[divan::bench]
fn day_01_part_one_zip() {
  day_01::part_one_zip(divan::black_box(include_str!("../assets/day-01/asset.txt")));
}

#[divan::bench]
fn day_01_part_one_insert_sort() {
  day_01::part_one_insert_sort(divan::black_box(include_str!("../assets/day-01/asset.txt")));
}

#[divan::bench]
fn day_01_part_two() {
  day_01::part_two(divan::black_box(include_str!("../assets/day-01/asset.txt")));
}

#[divan::bench]
fn day_01_part_two_directly_parsed() {
  day_01::part_two_directly_parsed(divan::black_box(include_str!("../assets/day-01/asset.txt")));
}

#[divan::bench]
fn day_02_part_one() {
  day_02::part_one(divan::black_box(include_str!("../assets/day-02/asset.txt")));
}

#[divan::bench]
fn day_02_part_two() {
  day_02::part_two(divan::black_box(include_str!("../assets/day-02/asset.txt")));
}


#[divan::bench]
fn day_03_part_one() {
  day_03::part_one(divan::black_box(include_str!("../assets/day-03/asset.txt")));
}

#[divan::bench]
fn day_03_part_one_no_regex() {
  day_03::part_one_no_regex(divan::black_box(include_str!("../assets/day-03/asset.txt")));
}

#[divan::bench]
fn day_03_part_two() {
  day_03::part_two(divan::black_box(include_str!("../assets/day-03/asset.txt")));
}

#[divan::bench]
fn day_03_part_two_no_regex() {
  day_03::part_two_no_regex(divan::black_box(include_str!("../assets/day-03/asset.txt")));
}

#[divan::bench]
fn day_03_part_two_o_n() {
  day_03::part_two_o_n(divan::black_box(include_str!("../assets/day-03/asset.txt")));
}
