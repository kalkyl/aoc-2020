use std::collections::HashMap;

fn main() {
    let input = vec![9, 3, 1, 0, 8, 4];
    let mut mem: HashMap<_, _> = input.iter().enumerate().map(|(i, x)| (*x, i)).collect();
    let mut prev = *input.get(input.len() - 1).unwrap();
    for i in input.len()..2020 {
        let current = match mem.get(&prev) {
            Some(p) => i - 1 - p,
            _ => 0,
        };
        mem.insert(prev, i - 1);
        prev = current;
    }
    println!("{:?}", prev);
}
