use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::zip;

fn main() {
    let mut res1 = 0u64;
    let mut res2 = 0u64;

    let mut vl: Vec<u64> = Vec::new();
    let mut vr: Vec<u64> = Vec::new();
    let mut cr: HashMap<u64, u64> = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();

        let l = iter.next().unwrap().parse::<u64>().unwrap();
        vl.push(l);

        let r = iter.next().unwrap().parse::<u64>().unwrap();
        vr.push(r);
        if let Some(c) = cr.get_mut(&r) {
            *c += 1;
        } else {
            cr.insert(r, 1);
        }
    }

    vl.sort();
    vr.sort();

    for (a, b) in zip(vl, vr) {
        res1 += a.abs_diff(b);
        if let Some(c) = cr.get(&a) {
            res2 += a * c;
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}
