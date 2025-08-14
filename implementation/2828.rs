fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let _n: usize = next().parse().unwrap();
    let m: usize = next().parse().unwrap();
    let j: usize = next().parse().unwrap();
    let apple: Vec<usize> = (0..j)
        .into_iter()
        .map(|_| next().parse().unwrap())
        .collect();
    
    let mut left: usize = 1;
    let mut right: usize = m;
    let mut result: usize = 0;
    for a in apple {
        if a < left {
            result += left - a;
            right -= left - a;
            left -= left - a;
        } else if a > right {
            result += a - right;
            left += a - right;
            right += a - right;
        }
    }
    print!("{result}");
}