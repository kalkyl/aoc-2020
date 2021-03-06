use std::io::{BufRead, BufReader, Error};
use std::{collections::HashMap, fs::File};
type Rule<'a> = Vec<(usize, &'a str)>;

fn rule_from_str(s: &str) -> Result<(&str, Rule), ()> {
    match s.split("s contain ").collect::<Vec<_>>()[..] {
        [key, contents] => Ok((
            key,
            contents
                .split(", ")
                .filter_map(
                    |s| match s.trim_matches(|c: char| !c.is_numeric()).parse::<usize>() {
                        Ok(n) => Some((
                            n,
                            s.trim_matches(|c: char| c.is_numeric() || c == ' ' || c == '.')
                                .trim_end_matches('s'),
                        )),
                        _ => None,
                    },
                )
                .collect(),
        )),
        _ => Err(()),
    }
}

fn count_recursive(rules: &HashMap<&str, Rule>, key: &str) -> usize {
    rules.get(key).unwrap().iter().fold(0, |acc, (qty, name)| {
        acc + *qty + *qty * count_recursive(rules, name)
    })
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("./input/7.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let rules: HashMap<_, _> = input.iter().filter_map(|s| rule_from_str(s).ok()).collect();
    let result = count_recursive(&rules, "shiny gold bag");
    println!("{}", result);
    Ok(())
}
