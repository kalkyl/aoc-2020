use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const FILE: &str = "./input/1.txt";
const SUM: i64 = 2020;

fn read_lines<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    BufReader::new(io)
        .lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let vec = read_lines(File::open(FILE)?)?;
    let p = vec
        .iter()
        .enumerate()
        .find_map(|(i, &x)| {
            vec.iter()
                .enumerate()
                .find(|(ii, &y)| i != *ii && x + y == SUM)
                .map(|(_, &y)| (x, y))
        })
        .map(|(x, y)| x * y);

    match p {
        Some(n) => println!("{:?}", n),
        None => println!("No combination found!"),
    }
    Ok(())
}
