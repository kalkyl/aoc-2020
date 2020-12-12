use std::io::{BufRead, BufReader, Error};
use std::{collections::HashSet, fs::File};

fn instruction_from_string(s: String) -> (String, i32) {
    match s.split(' ').collect::<Vec<_>>()[..] {
        [cmd, arg] => (cmd.to_owned(), arg.parse().unwrap()),
        _ => panic!("Invalid data"),
    }
}

fn run(instructions: &[(String, i32)]) -> i32 {
    let (mut line, mut acc) = (0, 0);
    let mut executed = HashSet::new();
    while executed.insert(line) {
        if let Some((cmd, arg)) = instructions.get(line as usize) {
            match cmd.as_str() {
                "acc" => {
                    acc += arg;
                    line += 1;
                }
                "jmp" => line += arg,
                _ => line += 1,
            }
        }
    }
    acc
}

fn main() -> Result<(), Error> {
    let instructions = BufReader::new(File::open("./input/8.txt")?)
        .lines()
        .map(|l| l.map(instruction_from_string))
        .collect::<Result<Vec<_>, _>>()?;
    let acc = run(&instructions);
    println!("{}", acc);
    Ok(())
}
