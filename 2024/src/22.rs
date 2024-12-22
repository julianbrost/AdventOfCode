use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let mut res1 = 0u64;
    let mut bananas = HashMap::new();

    for line in io::stdin().lines() {
        let mut x = line.unwrap().parse::<u64>().unwrap();
        let mut v = Vec::new();
        let mut u = HashSet::new();
        for _ in 0..2000 {
            let a = x % 10;
            x = (x ^ (x * 64)) % 16777216;
            x = (x ^ (x / 32)) % 16777216;
            x = (x ^ (x * 2048)) % 16777216;
            let b = x % 10;
            v.push((b as i64 - a as i64) as i8);
            let l = v.len();
            if l >= 4 {
                let t = (v[l - 4], v[l - 3], v[l - 2], v[l - 1]);
                if u.insert(t) {
                    *bananas.entry(t).or_insert(0u64) += b;
                }
            }
        }
        res1 += x;
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", bananas.values().max().unwrap());
}
