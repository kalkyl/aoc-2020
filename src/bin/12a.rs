use std::io::{BufRead, BufReader, Error};
use std::{f32::consts::PI, fs::File};

fn direction_from_string(s: String) -> (char, i32) {
    (
        s.chars().next().unwrap(),
        s.trim_start_matches(|c: char| !c.is_numeric())
            .parse()
            .unwrap(),
    )
}

fn run(directions: &[(char, i32)]) -> (i32, i32, i32) {
    directions
        .iter()
        .fold((0, 0, 0), |(n, e, c), (action, arg)| match action {
            'N' => (n - arg, e, c),
            'S' => (n + arg, e, c),
            'E' => (n, e + arg, c),
            'W' => (n, e - arg, c),
            'L' => (n, e, c - arg),
            'R' => (n, e, c + arg),
            _ => {
                let dn = (*arg as f32 * (c as f32 * (PI / 180.)).cos()) as i32;
                let de = (*arg as f32 * (c as f32 * (PI / 180.)).sin()) as i32;
                (n + dn, e + de, c)
            }
        })
}

fn main() -> Result<(), Error> {
    let directions = BufReader::new(File::open("./input/12.txt")?)
        .lines()
        .map(|l| l.map(|v| direction_from_string(v)))
        .collect::<Result<Vec<_>, _>>()?;
    let (north, east, _) = run(&directions);
    let distance = north.abs() + east.abs();
    println!("{:?}", distance);
    Ok(())
}
