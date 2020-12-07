use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};

fn main() -> Result<(), Error> {
    let expenses = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(InvalidData, e))))
        .collect::<Result<Vec<i32>, _>>()?;
    let result = expenses.iter().enumerate().find_map(|(i, &x)| {
        expenses
            .iter()
            .enumerate()
            .skip(i + 1)
            .find_map(|(ii, &y)| {
                expenses
                    .iter()
                    .skip(ii + 1)
                    .find(|&z| x + y + z == 2020)
                    .map(|&z| x * y * z)
            })
    });
    println!("{}", result.unwrap());
    Ok(())
}
