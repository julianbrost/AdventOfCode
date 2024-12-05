use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let mut res1 = 0u64;
    let mut res2 = 0u64;

    let mut rules: Vec<(u64, u64)> = Vec::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        if line.contains('|') {
            let mut iter = line.split('|');
            let a = iter.next().unwrap().parse::<u64>().unwrap();
            let b = iter.next().unwrap().parse::<u64>().unwrap();
            rules.push((a, b));
        } else if !line.is_empty() {
            let pages = Vec::from_iter(line.split(',').map(|x| x.parse::<u64>().unwrap()));
            let mut idx: HashMap<u64, usize> = HashMap::new();
            for (i, p) in pages.iter().enumerate() {
                assert!(!idx.contains_key(p));
                idx.insert(*p, i);
            }

            let mut pass = true;
            for (a, b) in &rules {
                if idx.contains_key(a) && idx.contains_key(b) {
                    pass &= idx[a] < idx[b];
                }
            }
            if pass {
                res1 += pages[pages.len() / 2];
            } else {
                // Day 5 and it's extra assumption time: for the n-th element of a topologically
                // sorted list being unambiguous, it has to be the only one having dependencies
                // to n-1 other elements in the list.
                let mut preds: HashMap<u64, HashSet<u64>> = HashMap::new();
                for (a, b) in &rules {
                    if idx.contains_key(a) && idx.contains_key(b) {
                        preds.entry(*b).or_insert(HashSet::new()).insert(*a);
                    }
                }
                for (page, pred) in preds {
                    if pred.len() == pages.len() / 2 {
                        res2 += page
                    }
                }
            }
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}
