use std::io::{BufRead, BufReader, Error};
use std::{collections::VecDeque, fs::File};

fn main() -> Result<(), Error> {
    let mut list = BufReader::new(File::open("./input/10.txt")?)
        .lines()
        .map(|l| l.map(|v| v.parse().unwrap()))
        .collect::<Result<Vec<usize>, _>>()?;
    list.sort_unstable();
    let (_, result) = list
        .iter()
        .fold(
            vec![(0, 1usize)].into_iter().collect::<VecDeque<_>>(),
            |mut fifo, x| {
                let sum = fifo
                    .iter()
                    .filter(|(v, _)| *x - v <= 3)
                    .map(|(_, s)| *s)
                    .sum();
                fifo.push_front((*x, sum));
                fifo.truncate(3);
                fifo
            },
        )
        .into_iter()
        .next()
        .unwrap();
    println!("{}", result);
    Ok(())
}
