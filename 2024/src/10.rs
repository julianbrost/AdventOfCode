use std::collections::VecDeque;
use std::io;

fn main() {
    let input: Vec<Vec<u8>> = Vec::from_iter(io::stdin().lines().map(|l| {
        l.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }));

    let n = input.len();
    let m = input[0].len();

    let mut res1 = 0u64;
    let mut res2 = 0u64;

    for i in 0..n {
        for j in 0..m {
            if input[i][j] == 0 {
                let mut paths = vec![vec![0u64; m]; n];
                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                paths[i][j] = 1;
                while let Some((i, j)) = queue.pop_front() {
                    let h = input[i][j];
                    if h == 9 {
                        res1 += 1;
                        res2 += paths[i][j];
                    }
                    for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        if let Some(ni) = i.checked_add_signed(di) {
                            if let Some(nj) = j.checked_add_signed(dj) {
                                if ni < n && nj < m && input[ni][nj] == h + 1 {
                                    if paths[ni][nj] == 0 {
                                        queue.push_back((ni, nj));
                                    }
                                    paths[ni][nj] += paths[i][j];
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
