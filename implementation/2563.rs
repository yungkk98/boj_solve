fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let mut area: Vec<Vec<i64>> = vec![vec![0; 101]; 101];
    let n: usize = next().parse().unwrap();
    for _ in 0..n {
        let x: usize = next().parse().unwrap();
        let y: usize = next().parse().unwrap();
        for i in 0..10 {
            for j in 0..10 {
                area[i + y][j + x] = 1;
            }
        }
    }

    let result: i64 = area.into_iter()
        .map(|t| t.iter().sum())
        .collect::<Vec<_>>()
        .iter()
        .sum();

    print!("{}", result);
}