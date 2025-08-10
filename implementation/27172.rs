use std::fmt::Write;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();
    let mut stdout: String = String::new();

    let n: usize = next().parse().unwrap();
    let mut score: Vec<i32> = vec![0; n];

    let mut max_card: usize = 0;

    let mut cards: Vec<usize> = vec![0; n];
    let mut card: Vec<bool> = vec![false; 1000001];
    let mut index: Vec<usize> = vec![std::usize::MAX; 1000001];
    for i in 0..n {
        let c: usize = next().parse().unwrap();
        cards[i] = c;
        card[c] = true;
        index[c] = i;
        max_card = std::cmp::max(max_card, c as usize);
    }

    for i in 0..n {
        let mut temp: usize = cards[i] * 2;
        while temp <= max_card {
            if card[temp] {
                score[i] += 1;
                score[index[temp]] -= 1;
            }
            temp += cards[i];
        }
    }
    for s in score.into_iter() {
        write!(stdout, "{} ", s).unwrap();
    }
    print!("{stdout}");
}