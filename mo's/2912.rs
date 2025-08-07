use std::fmt::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let c: usize = next().parse().unwrap();
    let hat_list: Vec<usize> = (0..n)
        .into_iter()
        .map(|_| next().parse().unwrap())
        .collect();
    
    let q: usize = next().parse().unwrap();
    let mut query_list: Vec<(usize, usize, usize)> = (0..q)
        .into_iter()
        .map(|i| (i, next().parse::<usize>().unwrap() - 1, next().parse::<usize>().unwrap() - 1))
        .collect();

    // Sort query
    let block_size = (n as f64).sqrt() as usize;
    query_list.sort_by(|a, b| {
        let a_block = a.1 / block_size;
        let b_block = b.1 / block_size;
        if a_block != b_block {
            a_block.cmp(&b_block)
        } else {
            a.2.cmp(&b.2)
        }
    });

    let mut result: Vec<(bool, usize)> = vec![(false, 0); q];
    let mut color_count: Vec<usize> = vec![0; c + 1];
    let mut start: usize = 0;
    let mut end: usize = 0;
    color_count[hat_list[0]] = 1;

    for query in query_list.into_iter() {
        while start < query.1 {
            color_count[hat_list[start]] -= 1;
            start += 1;
        }
        while start > query.1 {
            start -= 1;
            color_count[hat_list[start]] += 1;
        }
        while end > query.2 {
            color_count[hat_list[end]] -= 1;
            end -= 1;
        }
        while end < query.2 {
            end += 1;
            color_count[hat_list[end]] += 1;
        }
        
        let max_index = color_count.iter()
                        .enumerate()
                        .max_by_key(|&(_index, value)| value)
                        .map(|(index, _value)| index)
                        .unwrap();
        if color_count[max_index] > (end - start + 1) / 2 {
            result[query.0] = (true, max_index);
        }
    }

    for (v, c) in result.into_iter() {
        if v {
            writeln!(stdout, "yes {c}").unwrap();
        } else {
            writeln!(stdout, "no").unwrap();
        }
    }
    print!("{stdout}");
}