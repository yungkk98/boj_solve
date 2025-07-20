fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: i64 = next().parse().unwrap();
    print!("{}\n{}", n * (n - 1) / 2, 2);
}