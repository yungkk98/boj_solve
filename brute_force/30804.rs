fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let fruits: Vec<usize> = (0..n)
        .into_iter()
        .map(|_| next().parse().unwrap())
        .collect();
    let mut fruit_count: Vec<usize> = vec![0; 10];

    let mut result: usize = 1;
    let mut start: usize = 0;
    let mut end: usize = 0;
    fruit_count[fruits[0]] += 1;
    while start < n {
        while end < n {
            end += 1;
            if end >= n {
                break;
            }
            fruit_count[fruits[end]] += 1;
            if fruit_count.clone().into_iter().map(|x| if x != 0 {1} else {0}).collect::<Vec<_>>().iter().sum::<usize>() > 2 {
                fruit_count[fruits[end]] -= 1;
                end -= 1;
                result = std::cmp::max(result, end - start + 1);
                break;
            } else {
                result = std::cmp::max(result, end - start + 1);
            }
        }
        fruit_count[fruits[start]] -= 1;
        start += 1;
    }
    print!("{result}");
}