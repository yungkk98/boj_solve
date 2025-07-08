fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let mut numbers: Vec<i64> = (0..5)
        .into_iter()
        .map(|_| next().parse().unwrap())
        .collect();
    numbers.sort();

    println!("{}", numbers.iter().sum::<i64>() / 5_i64);
    println!("{}", numbers[2]);
}