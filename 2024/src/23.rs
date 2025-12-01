use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (a, b) = line.split_once('-').unwrap();
        edges
            .entry(a.to_string())
            .or_insert_with(HashSet::new)
            .insert(b.to_string());
        edges
            .entry(b.to_string())
            .or_insert_with(HashSet::new)
            .insert(a.to_string());
    }

    let mut loops: HashSet<(String, String, String)> = HashSet::new();
    for a in edges.keys() {
        for b in edges.get(a).unwrap() {
            if a == b {
                continue;
            }
            for c in edges.get(b).unwrap() {
                if a == c || b == c {
                    continue;
                }
                for d in edges.get(c).unwrap() {
                    if a == d {
                        let mut v = [a, b, c];
                        v.sort();
                        loops.insert((v[0].to_string(), v[1].to_string(), v[2].to_string()));
                    }
                }
            }
        }
    }

    let mut res1 = 0u64;
    for (a, b, c) in loops {
        if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
            // println!("{}-{}-{}", a, b, c);
            res1 += 1;
        }
    }

    println!("Part 1: {}", res1);
}
