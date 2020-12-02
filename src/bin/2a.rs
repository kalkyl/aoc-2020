use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const FILE: &str = "./input/2.txt";

struct Policy {
    min: usize,
    max: usize,
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
                    min: min.parse()?,
                    max: max.parse()?,
                    letter: letter.chars().nth(0).unwrap(),
                },
                password: Box::new((*password).to_owned()),
            }),
            _ => panic!("Invalid data"),
        }
    }
    fn is_valid(&self) -> bool {
        let matches = self.password.matches(self.policy.letter).count();
        matches >= self.policy.min && matches <= self.policy.max
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
