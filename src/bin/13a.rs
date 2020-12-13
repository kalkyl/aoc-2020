use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/13.txt")?
        .split_terminator(|c| c == '\n' || c == ',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let target = *input.get(0).unwrap();
    let mut buses: Vec<_> = input.iter().skip(1).map(|t| (t, t - target % t)).collect();
    buses.sort_unstable_by_key(|(_, t)| *t);
    let result = buses.get(0).map(|(id, t)| *id * *t).unwrap();
    println!("{}", result);
    Ok(())
}
