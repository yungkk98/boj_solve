use std::fmt::Write;
use std::collections::VecDeque;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let mut stack: VecDeque<i32> = VecDeque::new();
    let n: usize = next().parse().unwrap();
    for _ in 0..n {
        let q: usize = next().parse().unwrap();
        match q {
            1 => stack.push_front(next().parse().unwrap()),
            2 => writeln!(stdout, "{}", if stack.is_empty() {-1} else {stack.pop_front().unwrap() as i32}).unwrap(),
            3 => writeln!(stdout, "{}", stack.len()).unwrap(),
            4 => writeln!(stdout, "{}", if stack.is_empty() {1} else {0}).unwrap(),
            5 => writeln!(stdout, "{}", if stack.is_empty() {-1} else {*stack.front().unwrap() as i32}).unwrap(),
            _ => todo!(),
        }
    }

    print!("{stdout}");
}