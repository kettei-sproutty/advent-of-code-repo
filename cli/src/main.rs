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
      if day >= 1 && day <= 25 {
        Ok(Validation::Valid)
      } else {
        Ok(Validation::Invalid(
          "Day must be between 1 and 25".into(),
        ))
      }
    })
    .prompt()
    .expect("Cannot read day");

  let year_path = format!("../year-{}", year);
  let cargo_toml_path = format!("{}/Cargo.toml", year_path);
  let parsed = toml::from_str::<toml::Value>(&std::fs::read_to_string(&cargo_toml_path).expect("Cannot read Cargo.toml"))
    .expect("Cannot parse Cargo.toml");

  let mut cargo_toml = parsed.as_table().expect("Cargo.toml must be a table").clone();

  let bin_section = cargo_toml
    .entry("bin")
    .or_insert_with(|| toml::Value::Array(vec![]))
    .as_array_mut()
    .expect("bin must be an array");

  let bin_name = format!("day-{:02}", day);
  let bin_path = format!("src/day-{:02}.rs", day);

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

  let year_day_path = format!("{}/src/day-{:02}.rs", year_path, day);
  let template = include_str!("../templates/bin.rs");

  std::fs::write(year_day_path, template.replace("{{day}}", &day.to_string()))
    .expect("Cannot write day file");

  println!("Day {} added to year {}", day, year);
}
