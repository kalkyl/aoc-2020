use std::{collections::HashMap, fs::read_to_string, io::Error};

fn passport_from_str(s: &str) -> HashMap<&str, &str> {
    s.split_whitespace()
        .filter_map(|field| match (*field).split(':').collect::<Vec<_>>()[..] {
            [key, value] => Some((key, value)),
            _ => None,
        })
        .collect()
}

fn is_valid_field(key: &str, value: &str) -> bool {
    let is_in_range = |s: &str, min, max| match s.parse::<usize>() {
        Ok(n) => n >= min && n <= max,
        _ => false,
    };
    match key {
        "byr" => is_in_range(value, 1920, 2002),
        "iyr" => is_in_range(value, 2010, 2020),
        "eyr" => is_in_range(value, 2020, 2030),
        "hgt" => match value.trim_start_matches(char::is_numeric) {
            "cm" => is_in_range(value.trim_end_matches(|c: char| !c.is_numeric()), 150, 193),
            "in" => is_in_range(value.trim_end_matches(|c: char| !c.is_numeric()), 59, 76),
            _ => false,
        },
        "hcl" => {
            value.starts_with('#')
                && value.chars().count() == 7
                && value
                    .chars()
                    .skip(1)
                    .all(|c| c.is_numeric() || ('a'..='f').contains(&c))
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.chars().count() == 9 && value.chars().all(char::is_numeric),
        _ => false,
    }
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/4.txt")?;
    let passports: Vec<_> = input.split("\n\n").map(|s| passport_from_str(s)).collect();
    let required_fields = ["byr", "iyr", "eyr", "ecl", "hcl", "pid", "hgt"];
    let valid_passports = passports
        .iter()
        .filter(|&passport| {
            required_fields.iter().all(|&key| match passport.get(key) {
                Some(value) => is_valid_field(key, value),
                _ => false,
            })
        })
        .count();
    println!("{}", valid_passports);
    Ok(())
}
