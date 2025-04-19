fn read_and_seperate_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("input");
    let numbers = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();

    return numbers
}

fn main() {
    let n_m: Vec<i32> = read_and_seperate_numbers();
    let mut cards: Vec<Vec<i32>> = Vec::new();

    for _i in 0..n_m[0] {
        let mut temp = read_and_seperate_numbers();
        temp.sort();
        cards.push(temp);
    }

    let mut score = vec![0; n_m[0] as usize];
    for j in 0..n_m[1] {
        let mut first: Vec<usize> = Vec::new();
        let mut max_card: i32 = -1;

        for i in 0..n_m[0] {
            let card: i32 = cards[i as usize][j as usize];
            if card > max_card {
                max_card = card;
                first.clear();
                first.push(i as usize);
            } else if card == max_card {
                first.push(i as usize);
            }
        }

        for i in first {
            score[i] = score[i] + 1;
        }
    }

    let max_score = score.iter().max().unwrap();

    for i in 0..n_m[0] {
        if score[i as usize] == *max_score {
            print!("{} ", i + 1);
        }
    }
}