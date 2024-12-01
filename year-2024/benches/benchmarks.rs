use year_2024::day_01;

fn main() {
  divan::main();
}

#[divan::bench]
fn day_01_part_one() {
  day_01::part_one(divan::black_box(include_str!("../assets/day-01/asset.txt")));
}

#[divan::bench]
fn day_01_part_two() {
  day_01::part_two(divan::black_box(include_str!("../assets/day-01/asset.txt")));
}
