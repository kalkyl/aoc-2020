fn main() {
    let input = vec![9, 3, 1, 0, 8, 4];
    const N: usize = 30000000;
    let mut mem = vec![0; N];
    input.iter().enumerate().for_each(|(i, x)| mem[*x] = i + 1);
    let mut prev = *input.last().unwrap();
    for i in input.len()..N {
        let current = match mem[prev] {
            0 => 0,
            p => i - p,
        };
        mem[prev] = i;
        prev = current;
    }
    println!("{}", prev);
}
