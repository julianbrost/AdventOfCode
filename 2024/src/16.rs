use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::io;

fn main() {
    let input: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut queue: BTreeSet<(u64, (usize, usize), Direction)> = BTreeSet::new();
    let mut cost: HashMap<((usize, usize), Direction), u64> = HashMap::new();
    let mut dest = None;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == 'S' {
                queue.insert((0u64, (x, y), Direction::E));
                cost.insert(((x, y), Direction::E), 0);
            } else if input[x][y] == 'E' {
                dest = Some((x, y));
            }
        }
    }
    assert!(!queue.is_empty());
    let dest = dest.unwrap();
    let mut res1 = None;

    while let Some((c, (x, y), d)) = queue.pop_first() {
        if (x, y) == dest && res1 == None {
            res1 = Some(c);
        }

        for nd in [Direction::N, Direction::E, Direction::S, Direction::W] {
            let (nx, ny) = nd.step((x, y));
            if input[nx][ny] == '#' {
                continue;
            }
            let nc = if nd == d {
                c + 1
            } else if nd == d.opposite() {
                c + 2001
            } else {
                c + 1001
            };
            match cost.entry(((nx, ny), nd)) {
                Entry::Occupied(mut o) => {
                    let &oc = o.get();
                    if nc < oc {
                        queue.remove(&(oc, (nx, ny), nd));
                        queue.insert((nc, (nx, ny), nd));
                        o.insert(nc);
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(nc);
                    queue.insert((nc, (nx, ny), nd));
                }
            }
        }
    }
    let res1 = res1.unwrap();
    println!("Part 1: {}", res1);

    let mut best = HashSet::new();
    let mut queue = VecDeque::new();
    for d in [Direction::N, Direction::E, Direction::S, Direction::W] {
        if let Some(&c) = cost.get(&(dest, d)) {
            if c == res1 {
                queue.push_back((dest, d));
            }
        }
    }
    assert!(!queue.is_empty());
    while let Some(((x, y), d)) = queue.pop_front() {
        if !best.insert(((x, y), d)) {
            continue;
        }
        let &c = cost.get(&((x, y), d)).unwrap();
        let (px, py) = d.opposite().step((x, y));
        for pd in [Direction::N, Direction::E, Direction::S, Direction::W] {
            if let Some(&pc) = cost.get(&((px, py), pd)) {
                let s = if pd == d {
                    1
                } else if pd == d.opposite() {
                    2001
                } else {
                    1001
                };
                if c >= s && c - s == pc {
                    queue.push_back(((px, py), pd));
                }
            }
        }
    }
    let res2 = best
        .iter()
        .map(|&((x, y), _)| (x, y))
        .collect::<HashSet<_>>()
        .len();
    println!("Part 2: {}", res2);
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Hash, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }

    fn step(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::N => (x - 1, y),
            Direction::E => (x, y + 1),
            Direction::S => (x + 1, y),
            Direction::W => (x, y - 1),
        }
    }
}
