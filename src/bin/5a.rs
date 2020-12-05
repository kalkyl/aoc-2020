use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn mid(x: &(usize, usize)) -> usize {
    (x.1 - x.0) / 2 + 1
}

fn seat_id_from_str(s: &str) -> usize {
    let row = s.chars().take(7).fold((0, 127), |acc, c| match c {
        'F' => (acc.0, acc.1 - mid(&acc)),
        'B' => (acc.0 + mid(&acc), acc.1),
        _ => acc,
    });
    let col = s.chars().skip(7).fold((0, 7), |acc, c| match c {
        'L' => (acc.0, acc.1 - mid(&acc)),
        'R' => (acc.0 + mid(&acc), acc.1),
        _ => acc,
    });
    row.0 * 8 + col.0
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("./input/5.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let max_id = input.iter().map(|s| seat_id_from_str(s)).max().unwrap();
    println!("{}", max_id);
    Ok(())
}
