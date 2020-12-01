use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

const FILE: &str = "./input/1.txt";
const SUM: i32 = 3030;

fn read_lines<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    BufReader::new(io)
        .lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let vec = read_lines(File::open(FILE)?)?;
    let p = vec
        .iter()
        .enumerate()
        .find_map(|(i, &x)| {
            vec.iter().enumerate().skip(i + 1).find_map(|(ii, &y)| {
                vec.iter()
                    .skip(ii + 1)
                    .find(|&z| x + y + z == SUM)
                    .map(|&z| (x, y, z))
            })
        })
        .map(|(x, y, z)| x * y * z);

    match p {
        Some(n) => println!("{:?}", n),
        None => println!("No combination found!"),
    }

    Ok(())
}
