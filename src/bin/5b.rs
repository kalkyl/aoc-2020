use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn mid(a: usize, b: usize) -> usize {
    (b - a + 1) / 2
}

fn step((a, b): (usize, usize), c: char) -> (usize, usize) {
    match c {
        'F' | 'L' => (a, b - mid(a, b)),
        _ => (a + mid(a, b), b),
    }
}

fn seat_id_from_str(s: &str) -> usize {
    let row = s.chars().take(7).fold((0, 127), step).0;
    let col = s.chars().skip(7).fold((0, 7), step).0;
    row * 8 + col
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("./input/5.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let seats: Vec<_> = input.iter().map(|s| seat_id_from_str(s)).collect();
    let min_id = *seats.iter().min().unwrap();
    let max_id = *seats.iter().max().unwrap();
    let missing_id = (min_id..=max_id).find(|id| !seats.contains(id)).unwrap();
    println!("{}", missing_id);
    Ok(())
}
