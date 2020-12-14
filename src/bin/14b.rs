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
    let mut addresses = vec![String::with_capacity(36)];
    for (i, bit) in mask.chars().enumerate() {
        match bit {
            '1' => {
                for s in addresses.iter_mut() {
                    s.push('1');
                }
            }
            '0' => {
                let c = if addr & (1 << (35 - i)) > 0 { '1' } else { '0' };
                for s in addresses.iter_mut() {
                    s.push(c)
                }
            }
            _ => {
                let n = addresses.len();
                for i in 0..n {
                    addresses.push(addresses[i].clone());
                }
                for i in 0..n {
                    addresses[i].push('0');
                }
                for i in n..(2 * n) {
                    addresses[i].push('1');
                }
            }
        }
    }
    addresses
        .into_iter()
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
    let sum = mem.values().fold(0, |sum, v| sum + *v);
    println!("{}", sum);
    Ok(())
}
