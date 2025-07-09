use std::fmt::Write;
use std::collections::VecDeque;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let l: usize = next().parse().unwrap();
    let mut deque: VecDeque<(i64, usize)> = VecDeque::new();

    for i in 1..=n {
        let num: i64 = next().parse().unwrap();
        while !deque.is_empty() && deque.front().unwrap().1 + l - 1 < i {
            deque.pop_front();
        }

        while !deque.is_empty() && deque.back().unwrap().0 >= num {
            deque.pop_back();
        }
        
        deque.push_back((num, i));
        write!(stdout, "{} ", deque.front().unwrap().0);
    }

    print!("{stdout}");
}