fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let mut weight: Vec<i64> = (0..n)
        .into_iter()
        .map(|_| next().parse().unwrap())
        .collect();
    weight.sort();

    if weight[0] > 1 {
        print!("1");
        return;
    }

    let mut sum: i64 = weight[0];
    for v in weight[1..].into_iter() {
        if sum + 1 < *v {
            break;
        } else {
            sum += v;
        }
    }
    print!("{}", sum + 1);
}