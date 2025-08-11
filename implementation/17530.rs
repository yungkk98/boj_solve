fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let k: usize = next().parse().unwrap();
    for _ in 1..n {
        let c: usize = next().parse().unwrap();
        if c > k {
            print!("N");
            return;
        }
    }
    print!("S");
}