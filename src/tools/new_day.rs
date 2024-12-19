use aoc2024::{get_arg1, AocError, AocErrorType};
use std::error::Error;
use std::fs;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;
use toml::Table;

const RUST_TEMPLATE: &str = r##"use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"
    "#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 0);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

fn challenge1(_challenge_input: &str) -> i64 {
    42
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/{day_name}/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
"##;

fn main() -> Result<(), Box<dyn Error>> {
    let day_name = get_arg1().ok_or(AocError::new(AocErrorType::MissingArgument))?;

    let rust_file_name = day_name.to_string() + ".rs";

    let new_data_dir = format!("input_data/{}", &day_name);
    fs::create_dir_all(&new_data_dir)?;
    let new_src_dir = format!("src/{}", &day_name);
    fs::create_dir_all(&new_src_dir)?;
    fs::write(
        Path::new(&new_src_dir).join(&rust_file_name),
        RUST_TEMPLATE.replace("{day_name}", &day_name),
    )
    .expect("Couldn't write to output file");

    let toml_data = read_to_string("cargo.toml")?;
    let mut main_table = toml_data.parse::<Table>()?;
    let mut new_entry = Table::new();
    new_entry.insert("name".to_string(), day_name.clone().into());
    new_entry.insert(
        "path".to_string(),
        format!("src/{}/{}", day_name, rust_file_name).into(),
    );

    if let Some(bin_entries) = main_table.get_mut("bin") {
        let entries = bin_entries.as_array_mut().unwrap();
        entries.push(toml::Value::Table(new_entry));
    } else {
        main_table.insert("bin".to_string(), vec![new_entry].into());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("Cargo.toml")?;

    file.write_all(main_table.to_string().as_bytes())?;

    Ok(())
}
