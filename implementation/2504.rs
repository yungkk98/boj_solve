use std::collections::VecDeque;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let s: Vec<_> = next().chars().collect();
    let mut stack: VecDeque<i64> = VecDeque::new();
    let mut index: usize = 0;
    loop {
        let cur = s[index];
        if cur == '(' {
            stack.push_back(-1);
        } else if cur == '[' {
            stack.push_back(-2);
        } else if cur == ')' {
            let mut temp: i64 = 0;
            while !stack.is_empty() {
                let t = stack.pop_back().unwrap();
                if t > 0 {
                    temp += t;
                } else if t == -1 {
                    if temp == 0 {
                        stack.push_back(2);
                    } else {
                        stack.push_back(temp * 2);
                    }
                    break;
                } else if t == -2 {
                    print!("0");
                    return;
                }
            }
            if stack.is_empty() {
                print!("0");
                return;
            }
        } else if cur == ']' {
            let mut temp: i64 = 0;
            while !stack.is_empty() {
                let t = stack.pop_back().unwrap();
                if t > 0 {
                    temp += t;
                } else if t == -2 {
                    if temp == 0 {
                        stack.push_back(3);
                    } else {
                        stack.push_back(temp * 3);
                    }
                    break;
                } else if t == -1 {
                    print!("0");
                    return;
                }
            }
            if stack.is_empty() {
                print!("0");
                return;
            }
        }

        index += 1;
        if index >= s.len() {
            break;
        }
    }

    let mut result: i64 = 0;
    for v in stack.into_iter() {
        if v < 0 {
            print!("0");
            return;
        } else {
            result += v;
        }
    }
    print!("{}", result);
}