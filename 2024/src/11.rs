use std::collections::HashMap;
use std::io;

fn main() {
    let input: Vec<u64> = io::read_to_string(io::stdin())
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut s = Solver::new();

    println!(
        "Part 1: {}",
        input.iter().map(|&x| s.count(x, 25)).sum::<u64>()
    );
    println!(
        "Part 2: {}",
        input.iter().map(|&x| s.count(x, 75)).sum::<u64>()
    );
}

struct Solver {
    cache: HashMap<(u64, u64), u64>,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            cache: HashMap::new(),
        }
    }

    fn count(&mut self, stone: u64, remaining: u64) -> u64 {
        if let Some(c) = self.cache.get(&(stone, remaining)) {
            return *c;
        }

        let c = if remaining == 0 {
            1
        } else if stone == 0 {
            self.count(1, remaining - 1)
        } else {
            let (d, _) = (1..)
                .map(|e| (e, 10u64.pow(e)))
                .filter(|&(_, e)| e > stone)
                .next()
                .unwrap();

            if d % 2 == 0 {
                let p = 10u64.pow(d / 2);
                self.count(stone / p, remaining - 1) + self.count(stone % p, remaining - 1)
            } else {
                self.count(stone * 2024, remaining - 1)
            }
        };
        self.cache.insert((stone, remaining), c);
        c
    }
}
