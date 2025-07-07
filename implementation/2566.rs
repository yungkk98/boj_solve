fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let mut arr: Vec<Vec<u16>> = vec![vec![0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            arr[i][j] = next().parse().unwrap();
        }
    }

    let (mut y, mut x): (usize, usize) = (0, 0);
    let mut max: u16 = 0;
    for i in 0..9 {
        for j in 0..9 {
            if max < arr[i][j] {
                max = arr[i][j];
                y = i;
                x = j;
            }
        }
    }

    println!("{max}");
    println!("{} {}", y + 1, x + 1);
}