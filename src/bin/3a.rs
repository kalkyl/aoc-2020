use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let rows = BufReader::new(File::open("./input/3.txt")?)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;
    let trees = rows
        .iter()
        .enumerate()
        .filter(|(y, r)| r.chars().nth((y * 3) % r.chars().count()) == Some('#'))
        .count();
    println!("{}", trees);
    Ok(())
}
