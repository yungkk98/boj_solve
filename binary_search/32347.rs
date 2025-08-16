fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let n: usize = next().parse().unwrap();
    let k: usize = next().parse().unwrap();
    let mut power: Vec<u8> = (0..n)
        .into_iter()
        .map(|_| next().parse().unwrap())
        .collect();
    power[0] = 1;

    let mut low: usize = 1;
    let mut high: usize = 2000000;
    loop {
        let t = (low + high) / 2;
        let mut count: usize = 0;
        let mut cur: usize = n - 1;
        while count < k && cur != 0 {
            if cur < t {
                cur = 0;
                break;
            }

            let mut past_day: usize = cur - t;
            while power[past_day] != 1 {
                past_day += 1;
            }

            if past_day == cur {
                break;
            }
            count += 1;
            cur = past_day;
        }

        if cur == 0 {
            high = t;
        } else {
            low = t + 1;
        }

        if low == high {
            break;
        }
    }

    print!("{high}");
}