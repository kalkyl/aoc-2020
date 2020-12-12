use std::fs::File;
use std::io::{BufRead, BufReader, Error};

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
            'W' => (n, e - arg, c),
            'E' => (n, e + arg, c),
            'L' => (n, e, c - arg),
            'R' => (n, e, c + arg),
            _ => (
                n + (*arg as f32 * (c as f32).to_radians().cos()) as i32,
                e + (*arg as f32 * (c as f32).to_radians().sin()) as i32,
                c,
            ),
        })
}

fn main() -> Result<(), Error> {
    let directions = BufReader::new(File::open("./input/12.txt")?)
        .lines()
        .map(|l| l.map(direction_from_string))
        .collect::<Result<Vec<_>, _>>()?;
    let (north, east, _) = run(&directions);
    let distance = north.abs() + east.abs();
    println!("{:?}", distance);
    Ok(())
}
