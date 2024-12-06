use std::io::{self, BufRead};

fn main() {
    let mut input: Vec<Vec<char>> = Vec::from_iter(
        io::stdin()
            .lock()
            .lines()
            .map(|l| l.unwrap().chars().collect()),
    );

    let n = input.len() as isize;
    let m = input[0].len() as isize;

    let mut x = 0isize;
    let mut y = 0isize;

    for i in 0..n {
        for j in 0..m {
            if input[i as usize][j as usize] == '^' {
                x = i;
                y = j;
            }
        }
    }

    let res1 = count(&input, x, y);
    let mut res2 = 0u64;
    for i in 0..n {
        for j in 0..m {
            if input[i as usize][j as usize] == '.' {
                input[i as usize][j as usize] = '#';
                if count(&input, x, y) == 0 {
                    res2 += 1;
                }
                input[i as usize][j as usize] = '.';
            }
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

const DX: [isize; 4] = [-1, 0, 1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

fn count(input: &Vec<Vec<char>>, mut x: isize, mut y: isize) -> u64 {
    let n = input.len() as isize;
    let m = input[0].len() as isize;
    let l = (n * m) as u64;

    let mut steps = 0u64;
    let mut result = 0u64;
    let mut visited = vec![vec![false; m as usize]; n as usize];
    let mut d = 0;
    
    while (0..n).contains(&x) && (0..m).contains(&y) {
        if input[x as usize][y as usize] == '#' {
            x -= DX[d];
            y -= DY[d];
            d = (d + 1) % 4;
        }
        
        steps += 1;
        if steps > l {
            return 0;
        }

        if !visited[x as usize][y as usize] {
            visited[x as usize][y as usize] = true;
            result += 1;
        }

        x += DX[d];
        y += DY[d];
    }

    result
}
