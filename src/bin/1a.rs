use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expenses: Vec<i32> = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(InvalidData, e))))
        .collect::<Result<Vec<i32>, _>>()?;
    let result = expenses.iter().enumerate().find_map(|(i, &x)| {
        expenses
            .iter()
            .skip(i + 1)
            .find(|&y| x + y == 2020)
            .map(|&y| x * y)
    });
    match result {
        Some(n) => println!("{}", n),
        None => println!("No combination found!"),
    }
    Ok(())
}
