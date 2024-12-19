use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let patterns = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    let mut res1 = 0u64;
    let mut res2 = 0u64;
    for line in lines.map(|l| l.unwrap()).filter(|l| !l.is_empty()) {
        let mut possible = vec![0u64; line.len() + 1];
        possible[0] = 1;
        for i in 0..line.len() {
            if possible[i] > 0 {
                for p in &patterns {
                    let j = i + p.len();
                    if j <= line.len() && line[i..j] == p[..] {
                        possible[j] += possible[i];
                    }
                }
            }
        }
        if possible[line.len()] > 0 {
            res1 += 1;
            res2 += possible[line.len()];
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}
