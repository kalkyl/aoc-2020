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

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/4.txt")?;
    let passports: Vec<_> = input.split("\n\n").map(|s| passport_from_str(s)).collect();
    let valid_passports = passports
        .iter()
        .filter(|passport| passport.keys().filter(|&&key| key != "cid").count() == 7)
        .count();
    println!("{}", valid_passports);
    Ok(())
}
