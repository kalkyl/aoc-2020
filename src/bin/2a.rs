use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};

struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

struct Password<'a> {
    policy: Policy,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn from_str(s: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let v: Vec<&str> = s.split(|c| [' ', '-', ':'].contains(&c)).collect();
        match v.as_slice() {
            [min, max, letter, _, password] => Ok(Password {
                policy: Policy {
                    min: min.parse()?,
                    max: max.parse()?,
                    letter: letter.chars().next().unwrap(),
                },
                password,
            }),
            _ => Err(Box::new(Error::from(InvalidData))),
        }
    }
    fn is_valid(&self) -> bool {
        let matches = self.password.matches(self.policy.letter).count();
        matches >= self.policy.min && matches <= self.policy.max
    }
}

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/2.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let valid_passwords = entries
        .iter()
        .filter_map(|s| Password::from_str(s).ok())
        .filter(|p| p.is_valid())
        .count();
    println!("{}", valid_passwords);
    Ok(())
}
