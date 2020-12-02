use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct Policy {
    a: usize,
    b: usize,
    letter: char,
}

struct Password<'a> {
    policy: Policy,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn from_str(s: &'a str) -> Result<Self, std::num::ParseIntError> {
        let v: Vec<&str> = s.split(|c| [' ', '-', ':'].contains(&c)).collect();
        match v.as_slice() {
            [a, b, letter, _, password] => Ok(Password {
                policy: Policy {
                    a: a.parse()?,
                    b: b.parse()?,
                    letter: letter.chars().nth(0).unwrap(),
                },
                password,
            }),
            _ => panic!("Invalid data"),
        }
    }
    fn is_valid(&self) -> bool {
        self.password
            .char_indices()
            .filter(|(i, c)| {
                *c == self.policy.letter && (*i == self.policy.a - 1 || *i == self.policy.b - 1)
            })
            .count()
            == 1
    }
}

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/2.txt")?)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;
    let valid_passwords = entries
        .iter()
        .map(|s| Password::from_str(s).unwrap())
        .filter(|p| p.is_valid())
        .count();
    println!("{}", valid_passwords);
    Ok(())
}
