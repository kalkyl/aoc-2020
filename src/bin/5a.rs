use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn seat_id_from_str(s: &str) -> usize {
    let step = |(a, b), c| match c {
        'F' | 'L' => (a, b - (b - a + 1) / 2),
        _ => (a + (b - a + 1) / 2, b),
    };
    let row = s.chars().take(7).fold((0, 127), step).0;
    let col = s.chars().skip(7).fold((0, 7), step).0;
    row * 8 + col
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("./input/5.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let max_id = input.iter().map(|s| seat_id_from_str(s)).max().unwrap();
    println!("{}", max_id);
    Ok(())
}
