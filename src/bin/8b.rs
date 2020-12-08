use std::io::{BufRead, BufReader, Error};
use std::{collections::HashSet, fs::File};

fn instruction_from_str(s: &str) -> Result<(&str, i32), ()> {
    match s.split(' ').collect::<Vec<_>>().as_slice() {
        [cmd, arg] => Ok((cmd, arg.parse().map_err(|_| ())?)),
        _ => Err(()),
    }
}

fn run_flipped(instructions: &Vec<(&str, i32)>, flip_line: usize) -> Result<i32, ()> {
    let (line, acc) = (0..)
        .scan(
            (HashSet::new(), 0, 0),
            |(executed, line, acc), _| match executed.insert(*line) {
                true => {
                    match instructions.iter().nth(*line as usize).map(|(cmd, arg)| {
                        match (*line as usize == flip_line, *cmd) {
                            (true, "jmp") => ("nop", arg),
                            (true, "nop") => ("jmp", arg),
                            _ => (*cmd, arg),
                        }
                    }) {
                        Some((cmd, arg)) => match cmd {
                            "acc" => {
                                *acc += arg;
                                *line += 1;
                            }
                            "jmp" => {
                                *line += *arg;
                            }
                            _ => {
                                *line += 1;
                            }
                        },
                        _ => {
                            return None;
                        }
                    }
                    Some((*line, *acc))
                }
                _ => None,
            },
        )
        .last()
        .unwrap();
    match line as usize == instructions.len() {
        true => Ok(acc),
        _ => Err(()),
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("./input/8.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let instructions: Vec<_> = input
        .iter()
        .filter_map(|s| instruction_from_str(s).ok())
        .collect();
    let acc = instructions
        .iter()
        .enumerate()
        .filter(|(_, (cmd, _))| *cmd == "nop" || *cmd == "jmp")
        .find_map(|(i, _)| run_flipped(&instructions, i).ok());
    println!("{:?}", acc.unwrap());
    Ok(())
}
