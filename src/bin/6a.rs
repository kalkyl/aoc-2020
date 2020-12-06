use std::{collections::HashSet, fs::read_to_string, io::Error};

fn unique(s: &str) -> HashSet<char> {
    s.chars().filter(|&c| c != '\n').collect()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/6.txt")?;
    let result = input.split("\n\n").fold(0, |acc, s| acc + unique(s).len());
    println!("{}", result);
    Ok(())
}
