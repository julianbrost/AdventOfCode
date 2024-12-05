use std::io::{self, BufRead};

fn main() {
    let mut res1 = 0u64;
    let mut res2 = 0u64;

    let input: Vec<Vec<char>> = Vec::from_iter(
        io::stdin()
            .lock()
            .lines()
            .map(|l| l.unwrap().chars().collect()),
    );

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            for di in [-1, 0, 1] {
                for dj in [-1, 0, 1] {
                    if di != 0 || dj != 0 {
                        res1 += "XMAS".chars().enumerate().all(|(k, c)| {
                            let ii = (i as isize) + (k as isize) * di;
                            let jj = (j as isize) + (k as isize) * dj;
                            check(&input, ii, jj, c)
                        }) as u64;
                    }
                }
            }

            if input[i][j] == 'A' {
                let i = i as isize;
                let j = j as isize;
                if (check(&input, i - 1, j - 1, 'M') && check(&input, i + 1, j + 1, 'S')
                    || check(&input, i + 1, j + 1, 'M') && check(&input, i - 1, j - 1, 'S'))
                    && (check(&input, i - 1, j + 1, 'M') && check(&input, i + 1, j - 1, 'S')
                        || check(&input, i + 1, j - 1, 'M') && check(&input, i - 1, j + 1, 'S'))
                {
                    res2 += 1;
                }
            }
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn check(input: &Vec<Vec<char>>, i: isize, j: isize, c: char) -> bool {
    (0..(input.len() as isize)).contains(&i)
        && (0..(input[i as usize].len() as isize)).contains(&j)
        && input[i as usize][j as usize] == c
}
