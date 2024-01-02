use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut tree = HashMap::new();
    for _ in 0..n {
        let _name = lines.next();
        let city = lines.next().unwrap().unwrap();

        match tree.get_mut(&city) {
            Some(frequency) => *frequency += 1,
            None => {
                tree.insert(city, 1);
            }
        };
    }

    for (city, frequency) in tree.iter() {
        println!("{} {}", city, frequency);
    }
}
