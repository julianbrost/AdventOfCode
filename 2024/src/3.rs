use regex::Regex;
use std::io;

fn main() {
    let mut res1 = 0u64;
    let mut res2 = 0u64;

    let input = io::read_to_string(io::stdin()).unwrap();
    let re = Regex::new(r"(do)(\(\))|(do)(n't\(\))|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut enabled = true;
    for (_, [a, b]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
        if a == "do" {
            enabled = b == "()"
        } else {
            let p = a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
            res1 += p;
            if enabled {
                res2 += p;
            }
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}
