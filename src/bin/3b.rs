use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let rows = BufReader::new(File::open("./input/3.txt")?)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let trees = slopes
        .iter()
        .map(|(xs, ys)| {
            rows.iter()
                .enumerate()
                .filter(|(y, r)| {
                    (y % ys == 0)
                        && (r.chars().nth(((y / ys) * xs) % r.chars().count()) == Some('#'))
                })
                .count()
        })
        .fold(1, |acc, s| acc * s);
    println!("{:?}", trees);
    Ok(())
}
