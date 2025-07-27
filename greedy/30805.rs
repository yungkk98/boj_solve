use std::fmt::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let mut a: Vec<(u8, usize)> = (0..n)
        .into_iter()
        .map(|i| (next().parse().unwrap(), i))
        .collect();
    
    let m: usize = next().parse().unwrap();
    let mut b: Vec<(u8, usize)> = (0..m)
        .into_iter()
        .map(|i| (next().parse().unwrap(), i))
        .collect();

    a.sort_by(|f, s| if f.0 == s.0 {f.1.cmp(&s.1)} else {s.0.cmp(&f.0)});
    b.sort_by(|f, s| if f.0 == s.0 {f.1.cmp(&s.1)} else {s.0.cmp(&f.0)});

    let mut result: Vec<u8> = vec![];
    let mut a_val: (u8, usize) = a[0];
    let mut a_idx: usize = 0;
    let mut b_val: (u8, usize) = b[0];
    let mut b_idx: usize = 0;
    let mut a_last: usize = 0;
    let mut b_last: usize = 0;
    loop {
        if a_val.0 == b_val.0 {
            result.push(a_val.0);
            a_last = a_val.1;
            b_last = b_val.1;
            a_idx += 1;
            b_idx += 1;
        } else if a_val.0 > b_val.0 {
            a_idx += 1;
        } else {
            b_idx += 1;
        }

        while a_idx < n && a_last > a[a_idx].1 {
            a_idx += 1;
        }
        while b_idx < m && b_last > b[b_idx].1{
            b_idx += 1;
        }

        if a_idx >= n || b_idx >= m {
            break;
        }

        a_val = a[a_idx];
        b_val = b[b_idx];
    }

    writeln!(stdout, "{}", result.len()).unwrap();
    for v in result.into_iter() {
        write!(stdout, "{v} ").unwrap();
    }
    print!("{stdout}");
}