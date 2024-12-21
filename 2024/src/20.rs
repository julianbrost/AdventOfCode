use std::collections::VecDeque;
use std::io;

fn main() {
    let input: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let n = input.len();
    let m = input[0].len();

    let start = find(&input, 'S').unwrap();
    let end = find(&input, 'E').unwrap();

    let d_start = dist(&input, start);
    let d_end = dist(&input, end);

    let total = d_start[end.0][end.1].unwrap();
    assert_eq!(total, d_end[start.0][start.1].unwrap());

    let mut res1 = 0u64;
    let mut res2 = 0u64;
    for x1 in 0..n {
        for y1 in 0..m {
            for x2 in x1.checked_sub(20).unwrap_or(0)..=(x1 + 20).min(n - 1) {
                let dx = x1.abs_diff(x2);
                for y2 in (y1.checked_sub(20 - dx).unwrap_or(0))..=(y1 + 20 - dx).min(n - 1) {
                    if let Some(d1) = d_start[x1][y1] {
                        if let Some(d2) = d_end[x2][y2] {
                            let cheat = x1.abs_diff(x2) + y1.abs_diff(y2);
                            let d = d1 + cheat + d2;
                            if d + 100 <= total {
                                if cheat <= 2 {
                                    res1 += 1;
                                }
                                if cheat <= 20 {
                                    res2 += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn find(map: &Vec<Vec<char>>, c: char) -> Option<(usize, usize)> {
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == c {
                return Some((x, y));
            }
        }
    }
    None
}

fn dist(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<Vec<Option<usize>>> {
    let mut d = vec![vec![None; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    d[x][y] = Some(0);
    queue.push_back((x, y));
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(nx) = x.checked_add_signed(dx) {
                if let Some(ny) = y.checked_add_signed(dy) {
                    if nx < map.len()
                        && ny < map[nx].len()
                        && map[nx][ny] != '#'
                        && d[nx][ny].is_none()
                    {
                        d[nx][ny] = Some(d[x][y].unwrap() + 1);
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }
    d
}
