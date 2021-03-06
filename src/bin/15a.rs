use std::collections::HashMap;

fn main() {
    let input = vec![9, 3, 1, 0, 8, 4];
    let mut mem: HashMap<_, _> = input.iter().enumerate().map(|(i, x)| (*x, i + 1)).collect();
    let mut prev = *input.last().unwrap();
    for i in input.len()..2020 {
        let current = match mem.get(&prev) {
            Some(p) => i - p,
            _ => 0,
        };
        mem.insert(prev, i);
        prev = current;
    }
    println!("{}", prev);
}
