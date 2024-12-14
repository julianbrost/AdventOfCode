use regex::Regex;
use std::cmp::Ordering;
use std::io;

fn main() {
    let re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    let input = Vec::from_iter(io::stdin().lines().map(|l| {
        let l = l.unwrap();
        let [px, py, vx, vy] = re
            .captures(l.as_str())
            .map(|c| c.extract())
            .unwrap()
            .1
            .map(|x| x.parse::<i64>().unwrap());
        ((px, py), (vx, vy))
    }));

    // Crude auto-detection for different constants needed for the sample.
    let (n, m) = if input.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };
    let steps = 100;

    let mut qs = [0u64; 4];
    for &((px, py), (vx, vy)) in &input {
        let x = (px + steps * vx).rem_euclid(n);
        let y = (py + steps * vy).rem_euclid(m);
        match (x.cmp(&(n / 2)), y.cmp(&(m / 2))) {
            (Ordering::Less, Ordering::Less) => qs[0] += 1,
            (Ordering::Less, Ordering::Greater) => qs[1] += 1,
            (Ordering::Greater, Ordering::Less) => qs[2] += 1,
            (Ordering::Greater, Ordering::Greater) => qs[3] += 1,
            (_, _) => {}
        };
    }
    println!("Part 1: {}", qs.iter().product::<u64>());

    if n < 100 {
        println!("Part 2: (skipped, detected sample input)");
    } else {
        'outer: for i in 0.. {
            let mut occupied = vec![vec![false; m as usize]; n as usize];
            for ((px, py), (vx, vy)) in &input {
                let x = (px + i * vx).rem_euclid(n);
                let y = (py + i * vy).rem_euclid(m);
                occupied[x as usize][y as usize] = true;
            }

            // Additional assumption not given in the problem description: the Christmas tree is
            // surrounded by a rectangle of robots, so if there are 20 robots in a row, that's
            // probably the answer.
            for os in &occupied {
                let mut consecutive = 0;
                for &o in os {
                    if o {
                        consecutive += 1;
                        if consecutive >= 20 {
                            println!("Part 2: {}", i);
                            for y in 0..m {
                                for x in 0..n {
                                    if occupied[x as usize][y as usize] {
                                        print!("#");
                                    } else {
                                        print!(".");
                                    }
                                }
                                println!()
                            }
                            break 'outer;
                        }
                    } else {
                        consecutive = 0;
                    }
                }
            }
        }
    }
}
