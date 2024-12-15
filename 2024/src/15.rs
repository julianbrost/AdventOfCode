use std::collections::{HashSet, VecDeque};
use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let mut map1: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let moves: Vec<char> = lines
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .flatten()
        .collect();
    let mut map2: Vec<Vec<char>> = map1
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|&c| match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => {
                        panic!("unexpected map entry {}", c);
                    }
                })
                .collect()
        })
        .collect();

    let (mut x, mut y) = find_start(&map1).unwrap();
    for &mv in &moves {
        let mut d = 1;
        let (dx, dy) = match mv {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => {
                panic!("unexpected move {}", mv)
            }
        };

        while map1[add(x, d * dx)][add(y, d * dy)] == 'O' {
            d += 1;
        }

        if map1[add(x, d * dx)][add(y, d * dy)] == '.' {
            map1[add(x, d * dx)][add(y, d * dy)] = 'O';
            map1[x][y] = '.';
            (x, y) = (add(x, dx), add(y, dy));
            map1[x][y] = '@';
        }
    }

    let mut res1 = 0u64;
    for x in 0..map1.len() {
        for y in 0..map1[x].len() {
            if map1[x][y] == 'O' {
                res1 += 100 * x as u64 + y as u64;
            }
        }
    }
    println!("Part 1: {}", res1);

    let (mut x, mut y) = find_start(&map2).unwrap();
    for &mv in &moves {
        if mv == '<' || mv == '>' {
            let mut d = 1;
            let dy = if mv == '<' { -1 } else { 1 };

            while ['[', ']'].contains(&map2[x][add(y, d * dy)]) {
                d += 1;
            }

            if map2[x][add(y, d * dy)] == '.' {
                for i in (0..d).rev() {
                    map2[x][add(y, i * dy + dy)] = map2[x][add(y, i * dy)];
                }
                map2[x][y] = '.';
                y = add(y, dy);
            }
        } else {
            let dx = if mv == '^' { -1 } else { 1 };

            let mut mset: HashSet<(usize, usize)> = HashSet::new();
            let mut mqueue: VecDeque<(usize, usize)> = VecDeque::new();
            mset.insert((x, y));
            mqueue.push_back((add(x, dx), y));

            let mut possible = true;
            while let Some((nx, ny)) = mqueue.pop_front() {
                if map2[nx][ny] == '#' {
                    possible = false;
                    break;
                } else if map2[nx][ny] != '.' && mset.insert((nx, ny)) {
                    mqueue.push_back((add(nx, dx), ny));
                    if map2[nx][ny] == '[' {
                        assert_eq!(map2[nx][ny + 1], ']');
                        mqueue.push_back((nx, ny + 1));
                    }
                    if map2[nx][ny] == ']' {
                        assert_eq!(map2[nx][ny - 1], '[');
                        mqueue.push_back((nx, ny - 1));
                    }
                }
            }

            if possible {
                map2 = (0..map2.len())
                    .map(|nx| {
                        (0..map2[nx].len())
                            .map(|ny| {
                                if mset.contains(&(nx, ny)) && !mset.contains(&(add(nx, -dx), ny)) {
                                    '.'
                                } else if nx > 0 && mset.contains(&(add(nx, -dx), ny)) {
                                    map2[add(nx, -dx)][ny]
                                } else {
                                    map2[nx][ny]
                                }
                            })
                            .collect()
                    })
                    .collect();
                x = add(x, dx);
            }
        }
    }

    let mut res2 = 0u64;
    for x in 0..map2.len() {
        for y in 0..map2[x].len() {
            if map2[x][y] == '[' {
                res2 += 100 * x as u64 + y as u64;
            }
        }
    }
    println!("Part 2: {}", res2);
}

fn add(lhs: usize, rhs: isize) -> usize {
    lhs.checked_add_signed(rhs).unwrap()
}

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == '@' {
                return Some((x, y));
            }
        }
    }
    None
}
