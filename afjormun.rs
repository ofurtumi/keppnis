use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut chars = line.unwrap().chars().collect::<Vec<char>>();
        chars
            .iter_mut()
            .for_each(|c| *c = c.to_lowercase().nth(0).unwrap());
        chars[0] = chars[0].to_uppercase().nth(0).unwrap();
        println!("{}", chars.iter().collect::<String>());
    }
}
