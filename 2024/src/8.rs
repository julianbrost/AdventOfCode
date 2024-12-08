use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let input: Vec<Vec<char>> =
        Vec::from_iter(io::stdin().lines().map(|l| l.unwrap().chars().collect()));

    let ri = 0..(input.len() as isize);
    let rj = 0..(input[0].len() as isize);
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for i in ri.clone() {
        assert_eq!(input[i as usize].len(), rj.end as usize);
        for j in rj.clone() {
            let c = input[i as usize][j as usize];
            if c != '.' {
                antennas.entry(c).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    let mut antinodes1: HashSet<(isize, isize)> = HashSet::new();
    let mut antinodes2: HashSet<(isize, isize)> = HashSet::new();

    for (_, v) in &antennas {
        for &(ai, aj) in v {
            for &(bi, bj) in v {
                if (ai, aj) != (bi, bj) {
                    let di = bi - ai;
                    let dj = bj - aj;
                    let g = gcd(di, dj);
                    let di = di / g;
                    let dj = dj / g;

                    for s in 0.. {
                        let ni = bi + s * di;
                        let nj = bj + s * dj;

                        if ri.contains(&ni) && rj.contains(&nj) {
                            if s == g {
                                antinodes1.insert((ni, nj));
                            }
                            antinodes2.insert((ni, nj));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", antinodes1.len());
    println!("Part 1: {}", antinodes2.len());
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a.abs()
}
