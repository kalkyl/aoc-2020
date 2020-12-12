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

fn rotate((y, x): (i32, i32), angle: i32) -> (i32, i32) {
    let sin = (angle as f32).to_radians().sin();
    let cos = (angle as f32).to_radians().cos();
    (
        ((x as f32) * sin + (y as f32) * cos).round() as i32,
        ((x as f32) * cos - (y as f32) * sin).round() as i32,
    )
}

fn run(directions: &[(char, i32)]) -> ((i32, i32), (i32, i32)) {
    directions.iter().fold(
        ((0, 0), (1, 10)),
        |(mut ship, mut waypoint), (action, arg)| {
            match action {
                'N' => waypoint.0 += arg,
                'S' => waypoint.0 -= arg,
                'W' => waypoint.1 -= arg,
                'E' => waypoint.1 += arg,
                'R' => waypoint = rotate(waypoint, 360 - *arg),
                'L' => waypoint = rotate(waypoint, *arg),
                _ => {
                    ship.0 += *arg * waypoint.0;
                    ship.1 += *arg * waypoint.1;
                }
            }
            (ship, waypoint)
        },
    )
}

fn main() -> Result<(), Error> {
    let directions = BufReader::new(File::open("./input/12.txt")?)
        .lines()
        .map(|l| l.map(direction_from_string))
        .collect::<Result<Vec<_>, _>>()?;
    let ((north, east), _) = run(&directions);
    let distance = north.abs() + east.abs();
    println!("{:?}", distance);
    Ok(())
}
