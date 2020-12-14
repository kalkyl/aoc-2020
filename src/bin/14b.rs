use std::io::{BufRead, BufReader, Error};
use std::{collections::HashMap, fs::File};

enum Command {
    SetMask(String),
    WriteMem(u64, u64),
}

impl Command {
    fn from_string(s: String) -> Self {
        match s.starts_with("mem") {
            true => {
                match s
                    .split(" = ")
                    .map(|x| {
                        x.trim_matches(|c: char| !c.is_numeric())
                            .parse::<u64>()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()[..]
                {
                    [addr, value] => Self::WriteMem(addr, value),
                    _ => panic!("Invalid data"),
                }
            }
            _ => Self::SetMask(s.trim_start_matches("mask = ").to_owned()),
        }
    }
}

fn addresses(mask: &str, addr: u64) -> Vec<u64> {
    let mut list = vec![String::with_capacity(36)];
    for (i, bit) in mask.chars().enumerate() {
        match bit {
            '1' => list.iter_mut().for_each(|a| a.push('1')),
            '0' => list.iter_mut().for_each(|a| {
                a.push(match addr & (1 << (35 - i)) > 0 {
                    true => '1',
                    _ => '0',
                })
            }),
            _ => {
                let n = list.len();
                for i in 0..n {
                    list.push(list[i].clone());
                }
                list.iter_mut().take(n).for_each(|a| a.push('0'));
                list.iter_mut().skip(n).take(n).for_each(|a| a.push('1'));
            }
        }
    }
    list.into_iter()
        .map(|s| u64::from_str_radix(&s, 2).unwrap())
        .collect()
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
                    for a in addresses(&mask, *addr) {
                        mem.insert(a, *value);
                    }
                }
            }
            (mask, mem)
        },
    );
    let sum: u64 = mem.values().sum();
    println!("{}", sum);
    Ok(())
}
