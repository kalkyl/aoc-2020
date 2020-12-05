use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn mid(a: usize, b: usize) -> usize {
    (b - a) / 2 + 1
}

fn seat_id_from_str(s: &str) -> usize {
    let row = s.chars().take(7).fold((0, 127), |(a, b), c| match c {
        'F' => (a, b - mid(a, b)),
        'B' => (a + mid(a, b), b),
        _ => panic!("Invalid input"),
    });
    let col = s.chars().skip(7).fold((0, 7), |(a, b), c| match c {
        'L' => (a, b - mid(a, b)),
        'R' => (a + mid(a, b), b),
        _ => panic!("Invalid input"),
    });
    row.0 * 8 + col.0
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
