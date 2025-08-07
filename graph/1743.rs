use std::fmt::Write;
use std::collections::VecDeque;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let m: usize = next().parse().unwrap();
    let k: usize = next().parse().unwrap();
    let mut field: Vec<Vec<bool>> = vec![vec![false; m + 1]; n + 1];

    for _ in 0..k {
        let r: usize = next().parse().unwrap();
        let c: usize = next().parse().unwrap();
        field[r][c] = true;
    }

    let mut result: usize = 0;
    for i in 1..=n {
        for j in 1..=m {
            if field[i][j] {
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                field[i][j] = false;
                queue.push_back((i, j));
                let mut size: usize = 1;
                while !queue.is_empty() {
                    let (r, c) = queue.pop_front().unwrap();
                    if r > 1 && field[r - 1][c] {
                        field[r - 1][c] = false;
                        queue.push_back((r - 1, c));
                        size += 1;
                    }
                    if c < m  && field[r][c + 1] {
                        field[r][c + 1] = false;
                        queue.push_back((r, c + 1));
                        size += 1;
                    }
                    if c > 1  && field[r][c - 1] {
                        field[r][c - 1] = false;
                        queue.push_back((r, c - 1));
                        size += 1;
                    }
                    if r < n  && field[r + 1][c] {
                        field[r + 1][c] = false;
                        queue.push_back((r + 1, c));
                        size += 1;
                    }
                }

                result = std::cmp::max(result, size);
            }
        }
    }

    println!("{result}");
}