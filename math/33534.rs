fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = stdin.split_whitespace();
    let mut next = || tokens.next().unwrap();

    let f: f64 = next().parse().unwrap();
    let pi: f64 = 3.14159265358979;
    let result: f64 = (f / pi).powf(0.5) * 2.0 * pi;
    print!("{:.1}", (result * 10.0).ceil() / 10.0);
}