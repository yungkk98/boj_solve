use std::fmt::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let c: usize = next().parse().unwrap();
    let n: usize = next().parse().unwrap();
    let mut dp: Vec<usize> = vec![std::usize::MAX; c + 1];
    dp[0] = 0;

    let mut marketing: Vec<(usize, usize)> = (0..n)
        .into_iter()
        .map(|_| (next().parse().unwrap(), next().parse().unwrap()))
        .collect();
    marketing.sort_by(|&(a, b), &(c, d)| (d as f64 / c as f64).partial_cmp(&(b as f64 / a as f64)).unwrap());
    for (money, person) in marketing.into_iter() {
        for i in 0..c {
            let mut next: usize = i + person;
            if next > c {
                next = c
            }
            if dp[i] != std::usize::MAX {
                dp[next] = std::cmp::min(dp[next], dp[i] + money);
            }
        }
    }

    print!("{}", dp[c]);
}