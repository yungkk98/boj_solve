use std::fmt::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    loop {
        let flight = next();
        let mut n: usize = next().parse().unwrap();
        if flight == "#" {
            break;
        }

        loop {
            let t = next().chars().collect::<Vec<_>>()[0];
            let v: usize = next().parse().unwrap();
            match t {
                'B' => if n + v <= 68 {n += v},
                'C' => if n >= v {n -= v},
                'X' => break,
                _ => todo!(),
            }
        }
        writeln!(stdout, "{} {}", flight, n).unwrap();
    }

    print!("{stdout}");
}