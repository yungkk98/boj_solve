fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let _: usize = next().parse().unwrap();
    let s = next();
    let mut even: usize = 0;
    let mut odd: usize = 0;
    for c in s.chars().into_iter() {
        if c.to_digit(10).unwrap() % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    if even > odd {
        print!("0");
    } else if even < odd {
        print!("1");
    } else {
        print!("-1");
    }
}