use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const FILE: &str = "./input/1.txt";
const SUM: i32 = 2020;

fn read_lines<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    BufReader::new(io)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let expenses = read_lines(File::open(FILE)?)?;
    let result = expenses.iter().enumerate().find_map(|(i, &x)| {
        expenses
            .iter()
            .skip(i + 1)
            .find(|&y| x + y == SUM)
            .map(|&y| x * y)
    });

    match result {
        Some(n) => println!("{:?}", n),
        None => println!("No combination found!"),
    }

    Ok(())
}
