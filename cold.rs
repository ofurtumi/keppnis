use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let _ = lines.next();
    let n = lines
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .filter(|&x| x < 0)
        .count();
    println!("{}", n);
}
