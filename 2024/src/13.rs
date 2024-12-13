use regex::Regex;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let input = Vec::from_iter(
        re.find_iter(input.as_str())
            .map(|m| m.as_str().parse::<i64>().unwrap()),
    );

    let mut res1 = 0u64;
    let mut res2 = 0u64;

    for chunk in input.chunks(6) {
        if let &[ax, ay, bx, by, px, py] = chunk {
            let qx = 10000000000000 + px;
            let qy = 10000000000000 + py;

            res1 += solve((ax, ay), (bx, by), (px, py)).unwrap_or(0);
            res2 += solve((ax, ay), (bx, by), (qx, qy)).unwrap_or(0);
        } else {
            panic!("input does not contain multiple of 6 numbers");
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn solve((ax, ay): (i64, i64), (bx, by): (i64, i64), (px, py): (i64, i64)) -> Option<u64> {
    let c = px * by - py * bx;
    let d = ax * by - ay * bx;
    assert_ne!(d, 0);
    if c % d != 0 {
        return None;
    }

    let a = c / d;
    if a < 0 {
        return None;
    }

    let e = py - a * ay;
    if e % by != 0 {
        return None;
    }

    let b = e / by;
    if b < 0 {
        return None;
    }

    assert_eq!(a * ax + b * bx, px);
    assert_eq!(a * ay + b * by, py);
    Some(3 * a as u64 + b as u64)
}
