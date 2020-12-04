use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, io::Error};

fn passport_from_str(s: &str) -> HashMap<&str, &str> {
    s.split_whitespace()
        .filter_map(
            |field| match *field.split(':').collect::<Vec<_>>().as_slice() {
                [key, value] => Some((key, value)),
                _ => None,
            },
        )
        .collect()
}

fn is_valid_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => value
            .parse::<usize>()
            .map(|n| n >= 1920 && n <= 2002)
            .unwrap_or(false),
        "iyr" => value
            .parse::<usize>()
            .map(|n| n >= 2010 && n <= 2020)
            .unwrap_or(false),
        "eyr" => value
            .parse::<usize>()
            .map(|n| n >= 2020 && n <= 2030)
            .unwrap_or(false),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "hcl" => Regex::new(r#"#[0-9a-f]{6}"#).unwrap().is_match(value),
        "pid" => Regex::new(r#"^[0-9]{9}$"#).unwrap().is_match(value),
        "hgt" => Regex::new(r#"(\d+)(cm|in)"#)
            .unwrap()
            .captures(value)
            .map(|c| match (c.get(1), c.get(2)) {
                (Some(height), Some(unit)) => {
                    match (unit.as_str(), height.as_str().parse::<usize>()) {
                        ("cm", Ok(h)) => h >= 150 && h <= 193,
                        ("in", Ok(h)) => h >= 59 && h <= 76,
                        _ => false,
                    }
                }
                _ => false,
            })
            .unwrap_or(false),
        _ => true,
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
                Some(&value) => is_valid_field(key, value),
                None => false,
            })
        })
        .count();
    println!("{}", valid_passports);
    Ok(())
}
