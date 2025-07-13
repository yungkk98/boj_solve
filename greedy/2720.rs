use std::fmt::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let t: usize = next().parse().unwrap();
    let coins: Vec<i32> = vec![25, 10, 5, 1];
    for _ in 0..t {
        let mut price: i32 = next().parse().unwrap();
        for c in &coins {
            let count = price / *c;
            write!(stdout, "{count} ").unwrap();
            price -= *c * count;
        }
        write!(stdout, "\n").unwrap();
    }

    print!("{stdout}");
}