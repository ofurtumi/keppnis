use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let nk = lines
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut characters = vec![vec![0, 0]; nk[1]];

    for line in lines {
        let char = line
            .unwrap()
            .split("")
            .filter(|x| x != &"")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        for (i, j) in char.iter().enumerate() {
            if *j == 0 {
                characters[i][0] += 1;
            } else {
                characters[i][1] += 1;
            }
        }
    }

    for i in characters {
        print!(
            "{}",
            i.iter().position(|r| i.iter().min().unwrap() == r).unwrap()
        );
    }
}
