use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};

fn is_valid(list: &[i64], i: usize) -> bool {
    match list.get(i) {
        Some(sum) => {
            let fifo: Vec<_> = list.iter().skip(i - 25).take(25).collect();
            fifo.iter()
                .any(|x| fifo.iter().filter(|&y| y != x).any(|&y| sum - *x == *y))
        }
        _ => false,
    }
}

fn find_contiguous_sum(list: &[i64], target: i64) -> Option<(usize, usize)> {
    let sum_range = |start, end| list.iter().skip(start).take(1 + end - start).sum::<i64>();
    let (mut start, mut end) = (0, 0);
    while !(sum_range(start, end) == target && end - start > 1) && end < list.len() {
        match sum_range(start, end) {
            sum if sum < target => end += 1,
            sum if sum > target => start += 1,
            _ => {}
        }
    }
    match sum_range(start, end) == target && end - start > 1 {
        true => Some((start, end)),
        _ => None,
    }
}

fn main() -> Result<(), Error> {
    let list = BufReader::new(File::open("./input/9.txt")?)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(InvalidData, e))))
        .collect::<Result<Vec<i64>, _>>()?;
    let result = list
        .iter()
        .enumerate()
        .skip(25)
        .find(|(i, _)| !is_valid(&list, *i))
        .and_then(|(_, target)| {
            find_contiguous_sum(&list, *target).and_then(|(start, end)| {
                let range: Vec<_> = list.iter().skip(start).take(1 + end - start).collect();
                match (range.iter().min(), range.iter().max()) {
                    (Some(min), Some(max)) => Some(*min + *max),
                    _ => None,
                }
            })
        });
    println!("{}", result.unwrap());
    Ok(())
}
