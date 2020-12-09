use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};
const BUFFER_SIZE: usize = 25;

fn is_valid(list: &[i64], len: usize, i: usize) -> bool {
    match list.get(i) {
        Some(sum) => {
            let fifo: Vec<_> = list.iter().skip(i - len).take(len).collect();
            fifo.iter()
                .any(|x| fifo.iter().filter(|&y| y != x).any(|&y| sum - *x == *y))
        }
        _ => false,
    }
}

fn main() -> Result<(), Error> {
    let list = BufReader::new(File::open("./input/9.txt")?)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(InvalidData, e))))
        .collect::<Result<Vec<i64>, _>>()?;
    let (_, result) = list
        .iter()
        .enumerate()
        .skip(BUFFER_SIZE)
        .find(|(i, _)| !is_valid(&list, BUFFER_SIZE, *i))
        .unwrap();
    println!("{}", result);
    Ok(())
}
