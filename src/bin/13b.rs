use std::{fs::read_to_string, io::Error};

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut max, mut min) = (a, b);
    if min > max {
        std::mem::swap(&mut max, &mut min)
    }
    while max % min != 0 {
        let res = max % min;
        max = min;
        min = res;
    }
    min
}

fn main() -> Result<(), Error> {
    let buses = read_to_string("./input/13.txt")?
        .split_terminator(|c| c == '\n' || c == ',')
        .skip(1)
        .enumerate()
        .filter_map(|(i, s)| match s.parse::<u64>() {
            Ok(id) => Some((i, id)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let (_, result) = buses.iter().fold((1u64, 0u64), |(dt, mut t), (i, id)| {
        while (t + *i as u64) % id != 0 {
            t += dt;
        }
        (lcm(dt, *id), t)
    });
    println!("{}", result);
    Ok(())
}
