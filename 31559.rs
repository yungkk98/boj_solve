fn read_and_seperate_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("input");
    let numbers = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();

    return numbers
}

fn main() {
    let n_s = read_and_seperate_numbers();
    let n = n_s[0] as usize;
    let s = n_s[1] as usize;
    let mut v: i32 = 1;
    
    let mut line: Vec<(i32, i32)> = Vec::new();
    line.push((0, 0));
    for _i in 1..n+1 {
        let temp = read_and_seperate_numbers();
        line.push((temp[0], temp[1]));
    }

    let mut next_position: i32 = s as i32;
    let mut score: i32 = 0;
    let mut loop_count: i32 = 0;
    loop {
        if line[next_position as usize].0 == 1  && line[next_position as usize].1 <= v.abs(){
            score += 1;
            line[next_position as usize].0 = 2;
        } else if line[next_position as usize].0 == 0 {
            v *= -1;
            if v < 0 {
                v -= line[next_position as usize].1;
            } else {
                v += line[next_position as usize].1;
            }
        }

        next_position += v;
        if next_position > n as i32 || next_position <= 0 {
            break;
        }
        loop_count += 1;
        if loop_count > n as i32 * n as i32 {
            break;
        }
    }

    println!("{}", score);
}