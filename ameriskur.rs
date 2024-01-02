use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let n = line.trim().parse::<f32>().unwrap();
    print!("{}", n * 0.09144)
}
