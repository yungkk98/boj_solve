fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let mut result: f64 = 0_f64;
    for _ in 0..n {
        let (q, y): (f64, f64) = (next().parse().unwrap(), next().parse().unwrap());
        result += q * y;
    }
    print!("{:.3}", result);
}