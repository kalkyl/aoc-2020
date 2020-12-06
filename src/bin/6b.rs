use std::{collections::HashSet, fs::read_to_string, io::Error};

fn unique(s: &str) -> HashSet<char> {
    s.chars().filter(|&c| c != '\n').collect()
}

fn count_common(s: &str) -> usize {
    unique(s)
        .iter()
        .filter(|&&c| s.lines().all(|l| l.contains(c)))
        .count()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/6.txt")?;
    let result = input.split("\n\n").fold(0, |acc, s| acc + count_common(s));
    println!("{}", result);
    Ok(())
}
