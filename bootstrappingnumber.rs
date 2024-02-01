use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).ok();
    let n: f64 = buffer.trim().parse().unwrap();
    let new = oldton(n, 1e-7);
    println!("{new}");
}

fn oldton(n: f64, eps: f64) -> f64 {
    let mut start = 0.0;
    let mut end = n;
    while true {
        let half = start + (end - start) / 2.0;
        let pow = n.powf(1.0 / half);
        if pow + eps < half {
            end = half;
        } else if half < pow - eps {
            start = half;
        } else {
            return half;
        }
    }
    return 0.0;
}
