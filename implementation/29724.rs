fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let mut mass: i64 = 0;
    let mut pay: i64 = 0;
    for _ in 0..n {
        let t = next();
        let w: i64 = next().parse().unwrap();
        let h: i64 = next().parse().unwrap();
        let l: i64 = next().parse().unwrap();
        if t == "A" {
            let count = (w / 12) * (h / 12) * (l / 12);
            mass += count * 500 + 1000;
            pay += count * 4000;
        } else {
            mass += 120 * 50;
        }
    }
    print!("{}\n{}", mass, pay);
}