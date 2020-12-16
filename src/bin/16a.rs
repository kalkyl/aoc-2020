use std::{collections::HashMap, fs::File};
use std::{
    io::{BufRead, BufReader, Error},
    ops::RangeInclusive,
};

fn rules(input: &[String]) -> HashMap<&str, Vec<RangeInclusive<usize>>> {
    input
        .iter()
        .take(20)
        .map(|s| match s.split(": ").collect::<Vec<_>>()[..] {
            [key, value] => (
                key,
                value
                    .split(" or ")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|v| match v.split('-').collect::<Vec<_>>()[..] {
                        [min, max] => min.parse::<usize>().unwrap()..=max.parse::<usize>().unwrap(),
                        _ => panic!("Invalid input"),
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => panic!("Invalid input"),
        })
        .collect()
}

fn nearby(input: &[String]) -> Vec<Vec<usize>> {
    input
        .iter()
        .skip(25)
        .map(|s| {
            s.split(',')
                .collect::<Vec<_>>()
                .into_iter()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("./input/16.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let rules = rules(&input);
    let result: usize = nearby(&input)
        .iter()
        .flat_map(|n| {
            n.iter()
                .filter(|search| {
                    !rules
                        .iter()
                        .any(|(_, v)| v.iter().any(|r| r.contains(&search)))
                })
                .collect::<Vec<_>>()
        })
        .sum();
    println!("{:?}", result);
    Ok(())
}
