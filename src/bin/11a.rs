use std::{fs::read_to_string, io::Error};

fn count_adjacent(map: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    ((row as i32 - 1).max(0) as usize..=(row + 1).min(map.len() - 1))
        .map(|y| {
            ((col as i32 - 1).max(0) as usize..=(col + 1).min(map[0].len() - 1))
                .filter(|x| match (map[y][*x], (y, *x) == (row, col)) {
                    ('#', false) => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/11.txt")?;
    let mut map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut prev_map = vec![vec![' '; map[0].len()]; map.len()];
    while map != prev_map {
        prev_map = map.clone();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                match (map[y][x], count_adjacent(&prev_map, y, x)) {
                    ('L', 0) => map[y][x] = '#',
                    ('#', n) if n > 3 => map[y][x] = 'L',
                    _ => (),
                }
            }
        }
    }
    let result: usize = map
        .iter()
        .flat_map(|l| l.iter().filter(|&c| *c == '#'))
        .count();
    println!("{}", result);
    Ok(())
}
