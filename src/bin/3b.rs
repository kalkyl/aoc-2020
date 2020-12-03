use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn count_trees(rows: &Vec<String>, slope: &(usize, usize)) -> usize {
    let (xs, ys) = slope;
    rows.iter()
        .enumerate()
        .filter(|(y, r)| {
            let x = (xs * (y / ys)) % r.chars().count();
            (y % ys == 0) && (r.chars().nth(x) == Some('#'))
        })
        .count()
}

fn main() -> Result<(), Error> {
    let rows = BufReader::new(File::open("./input/3.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result = slopes
        .iter()
        .fold(1, |acc, slope| acc * count_trees(&rows, slope));
    println!("{}", result);
    Ok(())
}
