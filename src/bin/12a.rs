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
        .fold((0, 0, 0), |(e, n, course), (action, arg)| match action {
            'E' => (e + arg, n, course),
            'W' => (e - arg, n, course),
            'N' => (e, n + arg, course),
            'S' => (e, n - arg, course),
            'R' => (e, n, course + arg),
            'L' => (e, n, course - arg),
            _ => (
                e + (*arg as f32 * (course as f32).to_radians().sin()) as i32,
                n - (*arg as f32 * (course as f32).to_radians().cos()) as i32,
                course,
            ),
        })
}

fn main() -> Result<(), Error> {
    let directions = BufReader::new(File::open("./input/12.txt")?)
        .lines()
        .map(|l| l.map(direction_from_string))
        .collect::<Result<Vec<_>, _>>()?;
    let (east, north, _) = run(&directions);
    let distance = east.abs() + north.abs();
    println!("{:?}", distance);
    Ok(())
}
