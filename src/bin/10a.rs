use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};

fn main() -> Result<(), Error> {
    let mut list = BufReader::new(File::open("./input/10.txt")?)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(InvalidData, e))))
        .collect::<Result<Vec<i32>, _>>()?;
    list.sort_unstable();
    list.dedup();
    let (diff1, diff3): (Vec<_>, Vec<_>) = list
        .iter()
        .scan(0, |prev_rating, rating| {
            let diff = *rating - *prev_rating;
            *prev_rating = *rating;
            match diff >= 1 && diff <= 3 {
                true => Some(diff),
                _ => None,
            }
        })
        .filter(|diff| *diff != 2)
        .partition(|diff| *diff == 1);
    let result = diff1.len() * (diff3.len() + 1);
    println!("{}", result);
    Ok(())
}
