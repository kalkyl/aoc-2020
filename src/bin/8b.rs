use std::io::{BufRead, BufReader, Error};
use std::{collections::HashSet, fs::File};

fn instruction_from_string(s: String) -> (String, i32) {
    match s.split(' ').collect::<Vec<_>>()[..] {
        [cmd, arg] => (cmd.to_owned(), arg.parse().unwrap()),
        _ => panic!("Invalid data"),
    }
}

fn run_flipped(instructions: &[(String, i32)], flip_line: usize) -> Result<i32, ()> {
    let (mut line, mut acc) = (0i32, 0i32);
    let mut executed = HashSet::new();
    while executed.insert(line) {
        if let Some((cmd, arg)) = instructions.get(line as usize) {
            match (cmd.as_str(), line as usize == flip_line) {
                ("jmp", false) | ("nop", true) => line += *arg,
                ("nop", false) | ("jmp", true) => line += 1,
                _ => {
                    acc += *arg;
                    line += 1;
                }
            }
        }
    }
    match line as usize == instructions.len() {
        true => Ok(acc),
        _ => Err(()),
    }
}

fn main() -> Result<(), Error> {
    let instructions = BufReader::new(File::open("./input/8.txt")?)
        .lines()
        .map(|l| l.map(instruction_from_string))
        .collect::<Result<Vec<_>, _>>()?;
    let acc = instructions
        .iter()
        .enumerate()
        .filter(|(_, (cmd, _))| *cmd == "nop" || *cmd == "jmp")
        .find_map(|(i, _)| run_flipped(&instructions, i).ok());
    println!("{}", acc.unwrap());
    Ok(())
}
