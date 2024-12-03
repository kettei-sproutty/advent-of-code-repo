use inquire::{validator::Validation, CustomType};

fn main() {
  let year: usize = CustomType::new("Year: ")
    .with_validator(move |&year: &usize| {
      if year >= 2023 {
        Ok(Validation::Valid)
      } else {
        Ok(Validation::Invalid(
          "Year must be greater than or equal to 2023".into(),
        ))
      }
    })
    .prompt()
    .expect("Cannot read year");

  let day: usize = CustomType::new("Day: ")
    .with_validator(move |&day: &usize| {
      if (1..=25).contains(&day) {
        Ok(Validation::Valid)
      } else {
        Ok(Validation::Invalid(
          "Day must be between 1 and 25".into(),
        ))
      }
    })
    .prompt()
    .expect("Cannot read day");

  let year_path = format!("year-{year}");
  let cargo_toml_path = format!("{year_path}/Cargo.toml");
  let toml = std::fs::read_to_string(&cargo_toml_path).expect("Cannot read Cargo.toml");

  let parsed = toml::from_str::<toml::Value>(&toml).expect("Cannot parse Cargo.toml");

  let mut cargo_toml = parsed.as_table().expect("Cargo.toml must be a table").clone();

  let bin_section = cargo_toml
    .entry("bin")
    .or_insert_with(|| toml::Value::Array(vec![]))
    .as_array_mut()
    .expect("bin must be an array");

  let bin_name = format!("day-{day:02}");
  let bin_path = format!("src/day-{day:02}.rs");

  bin_section.push(toml::Value::Table(
    vec![
      ("name".to_string(), toml::Value::String(bin_name)),
      ("path".to_string(), toml::Value::String(bin_path)),
    ]
    .into_iter()
    .collect(),
  ));

  std::fs::write(cargo_toml_path, toml::to_string(&cargo_toml).expect("Cannot serialize Cargo.toml"))
    .expect("Cannot write Cargo.toml");

  let year_day_path = format!("{year_path}/src/day_{day:02}.rs");
  let template = include_str!("../templates/bin.rs");

  std::fs::write(year_day_path, template.replace("{{day}}", &format!("{day:02}")))
    .expect("Cannot write day file");

  std::fs::create_dir_all(format!("{year_path}/assets/day-{day:02}"))
    .expect("Cannot create asset directory");

  std::fs::write(
    format!("{year_path}/assets/day-{day:02}/asset.txt"),
    "".as_bytes(),
  ).expect("Cannot create asset file");

  std::fs::write(
    format!("{year_path}/assets/day-{day:02}/example.txt"),
    "".as_bytes(),
  ).expect("Cannot create asset file");

  println!("Day {day} added to year {year}");
}
