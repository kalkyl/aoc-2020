use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let rows = BufReader::new(File::open("./input/3.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let trees = rows
        .iter()
        .enumerate()
        .filter(|(y, r)| r.chars().nth((3 * y) % r.chars().count()) == Some('#'))
        .count();
    println!("{}", trees);
    Ok(())
}
