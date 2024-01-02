use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    println!("+{}+", "-".repeat(n));
    for _ in 0..n {
        println!("|{}|", " ".repeat(n));
    }
    println!("+{}+", "-".repeat(n));
}
