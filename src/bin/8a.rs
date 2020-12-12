use std::io::{BufRead, BufReader, Error};
use std::{collections::HashSet, fs::File};

fn instruction_from_str(s: &str) -> Result<(&str, i32), ()> {
    match s.split(' ').collect::<Vec<_>>()[..] {
        [cmd, arg] => Ok((cmd, arg.parse().map_err(|_| ())?)),
        _ => Err(()),
    }
}

fn run(instructions: &[String]) -> Option<i32> {
    (0..)
        .scan(
            (HashSet::new(), 0, 0),
            |(executed, line, acc), _| match executed.insert(*line) {
                true => {
                    match instructions
                        .get(*line as usize)
                        .and_then(|s| instruction_from_str(s).ok())
                    {
                        Some((cmd, arg)) => match cmd {
                            "acc" => {
                                *acc += arg;
                                *line += 1;
                            }
                            "jmp" => *line += arg,
                            _ => *line += 1,
                        },
                        _ => return None,
                    }
                    Some(*acc)
                }
                _ => None,
            },
        )
        .last()
}

fn main() -> Result<(), Error> {
    let instructions = BufReader::new(File::open("./input/8.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let acc = run(&instructions);
    println!("{}", acc.unwrap());
    Ok(())
}
