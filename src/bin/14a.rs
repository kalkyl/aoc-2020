use std::io::{BufRead, BufReader, Error};
use std::{collections::HashMap, fs::File};

enum Command {
    SetMask(String),
    WriteMem(usize, usize),
}

impl Command {
    fn from_string(s: String) -> Self {
        match s.starts_with("mem") {
            true => {
                match s
                    .split(" = ")
                    .map(|x| {
                        x.trim_matches(|c: char| !c.is_numeric())
                            .parse::<usize>()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()[..]
                {
                    [addr, value] => Self::WriteMem(addr as usize, value),
                    _ => panic!("Invalid data"),
                }
            }
            _ => Self::SetMask(s.trim_start_matches("mask = ").to_owned()),
        }
    }
}

fn apply_mask(mask: &str, value: usize) -> usize {
    value & usize::from_str_radix(&mask.replace('X', "1"), 2).unwrap()
        | usize::from_str_radix(&mask.replace('X', "0"), 2).unwrap()
}

fn main() -> Result<(), Error> {
    let commands = BufReader::new(File::open("./input/14.txt")?)
        .lines()
        .map(|l| l.map(Command::from_string))
        .collect::<Result<Vec<_>, _>>()?;
    let (_, mem) = commands.iter().fold(
        (String::with_capacity(36), HashMap::new()),
        |(mut mask, mut mem), cmd| {
            match cmd {
                Command::SetMask(m) => mask.clone_from(m),
                Command::WriteMem(addr, value) => {
                    mem.insert(addr, apply_mask(&mask, *value));
                }
            }
            (mask, mem)
        },
    );
    let sum: usize = mem.values().sum();
    println!("{}", sum);
    Ok(())
}
