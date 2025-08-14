fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let mut nums: Vec<i32> = vec![next().parse().unwrap(), next().parse().unwrap(),next().parse().unwrap()];
    nums.sort();
    print!("{} {} {}", nums[0], nums[1], nums[2]);
}