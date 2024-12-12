use std::collections::VecDeque;
use std::io;

fn main() {
    let input: Vec<Vec<char>> =
        Vec::from_iter(io::stdin().lines().map(|l| l.unwrap().chars().collect()));

    let mut res1 = 0u64;
    let mut res2 = 0u64;

    let n = input.len();
    let m = input[0].len();
    let mut visited = vec![vec![false; m]; n];

    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] {
                let mut area = 0u64;
                let mut perimeter = 0u64;
                let mut edges = 0u64;

                visited[i][j] = true;
                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                while let Some((i, j)) = queue.pop_front() {
                    let c = input[i][j];
                    area += 1;

                    for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        if let (Some(ni), Some(nj)) = (bounded_add(i, di, n), bounded_add(j, dj, m))
                        {
                            if input[ni][nj] != c {
                                perimeter += 1;
                            } else if !visited[ni][nj] {
                                visited[ni][nj] = true;
                                queue.push_back((ni, nj));
                            }
                        } else {
                            perimeter += 1;
                        }
                    }

                    const DS: [(isize, isize); 8] = [
                        (0, 1),
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, -1),
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                    ];

                    for d in (0..DS.len()).step_by(2) {
                        let mut same = [false, false, false];
                        for k in 0..3 {
                            let (di, dj) = DS[(d + k) % DS.len()];
                            if let (Some(ni), Some(nj)) =
                                (bounded_add(i, di, n), bounded_add(j, dj, m))
                            {
                                same[k] = input[ni][nj] == c;
                            }
                        }

                        edges += match same {
                            [false, _, false] => 1,
                            [true, false, true] => 1,
                            [_, _, _] => 0,
                        };
                    }
                }

                res1 += area * perimeter;
                res2 += area * edges;
            }
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn bounded_add(a: usize, b: isize, l: usize) -> Option<usize> {
    if let Some(x) = a.checked_add_signed(b) {
        if x < l {
            return Some(x);
        }
    }
    None
}
