use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const FILE: &str = "./input/2.txt";

struct Policy {
    a: usize,
    b: usize,
    letter: char,
}

struct Password {
    policy: Policy,
    password: Box<String>,
}

impl Password {
    fn from_string(record: String) -> Result<Password, std::num::ParseIntError> {
        let v: Vec<&str> = record.split(|c| [' ', '-', ':'].contains(&c)).collect();
        match v.as_slice() {
            [min, max, letter, _, password] => Ok(Password {
                policy: Policy {
                    a: min.parse()?,
                    b: max.parse()?,
                    letter: letter.chars().nth(0).unwrap(),
                },
                password: Box::new((*password).to_owned()),
            }),
            _ => panic!("Invalid data"),
        }
    }
    fn is_valid(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .filter(|(i, c)| {
                *c == self.policy.letter && (*i == self.policy.a - 1 || *i == self.policy.b - 1)
            })
            .count()
            == 1
    }
}

fn read_lines<'a, R: Read>(io: R) -> Result<Vec<Password>, Error> {
    BufReader::new(io)
        .lines()
        .map(|l| {
            l.and_then(|v| {
                Password::from_string(v).map_err(|e| Error::new(ErrorKind::InvalidData, e))
            })
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let passwords = read_lines(File::open(FILE)?)?;
    println!("{}", passwords.iter().filter(|p| p.is_valid()).count());
    Ok(())
}
