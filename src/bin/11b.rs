use std::{fs::read_to_string, io::Error};

fn count_visible(map: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut sum = 0;
    for dy in -1i32..=1 {
        'dx: for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let (mut y, mut x) = (row as i32 + dy, col as i32 + dx);
            while y >= 0 && (y as usize) < map.len() && x >= 0 && (x as usize) < map[0].len() {
                match map[y as usize][x as usize] {
                    '#' => {
                        sum += 1;
                        continue 'dx;
                    }
                    'L' => continue 'dx,
                    _ => (),
                }
                y += dy;
                x += dx;
            }
        }
    }
    sum
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/11.txt")?;
    let mut map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut prev_map = vec![vec![' '; map[0].len()]; map.len()];
    while map != prev_map {
        prev_map = map.clone();
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                match (map[y][x], count_visible(&prev_map, y, x)) {
                    ('L', 0) => map[y][x] = '#',
                    ('#', n) if n >= 5 => map[y][x] = 'L',
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
