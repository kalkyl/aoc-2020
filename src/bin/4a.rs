use std::{fs::read_to_string, io::Error, collections::HashMap};

fn parse(s: &str) -> HashMap<String, String> {
    s.split_whitespace()
        .filter_map(|f| match f.split(':').collect::<Vec<_>>().as_slice() {
            [key, value] => Some(((*key).to_owned(), (*value).to_owned())),
            _ => None,
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/4.txt")?;
    let passports: Vec<_> = input
        .split("\n\n")
        .map(|x| parse(x))
        .collect();
    let valid_passports = passports
        .iter()
        .filter(|p| p.keys().filter(|&k| k != "cid").count() == 7)
        .count();
    println!("{:?}", valid_passports);
    Ok(())
}
