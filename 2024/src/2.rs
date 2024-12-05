use std::io::{self, BufRead};

fn main() {
    let mut res1 = 0u64;
    let mut res2 = 0u64;

    for line in io::stdin().lock().lines() {
        let report = Vec::from_iter(
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap()),
        );

        let mut valid = false;
        let mut fixable = false;

        for first in [0, 1] {
            for dir in [-1, 1] {
                let mut cur_valid = first == 0;
                let mut dir_fixable = true;

                let mut prev = report[first];
                for cur in report.iter().skip(first + 1) {
                    let range = if dir > 0 {
                        (prev + dir)..=(prev + 3 * dir)
                    } else {
                        (prev + 3 * dir)..=(prev + dir)
                    };
                    if range.contains(cur) {
                        prev = *cur;
                    } else if !cur_valid {
                        dir_fixable = false;
                    } else {
                        cur_valid = false;
                    }
                }

                assert!(!cur_valid || dir_fixable);
                valid |= cur_valid;
                fixable |= dir_fixable;
            }
        }

        assert!(!valid || fixable);

        res1 += valid as u64;
        res2 += fixable as u64;
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}
