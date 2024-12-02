use year_2024::{day_01, day_02};

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
