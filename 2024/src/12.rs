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
                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                visited[i][j] = true;

                let mut area = 0u64;
                let mut perimeter = 0u64;
                let mut edges = 0u64;

                while let Some((i, j)) = queue.pop_front() {
                    let c = input[i][j];
                    area += 1;

                    for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let ni = i.checked_add_signed(di);
                        let nj = j.checked_add_signed(dj);
                        if get(&input, ni, nj) != Some(c) {
                            perimeter += 1;
                        } else {
                            if !visited[ni.unwrap()][nj.unwrap()] {
                                visited[ni.unwrap()][nj.unwrap()] = true;
                                queue.push_back((ni.unwrap(), nj.unwrap()));
                            }
                        }
                    }

                    for (di, dj) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        edges += match [
                            get(&input, Some(i), j.checked_add_signed(dj)) == Some(c),
                            get(&input, i.checked_add_signed(di), j.checked_add_signed(dj))
                                == Some(c),
                            get(&input, i.checked_add_signed(di), Some(j)) == Some(c),
                        ] {
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

fn get(input: &Vec<Vec<char>>, i: Option<usize>, j: Option<usize>) -> Option<char> {
    if let (Some(i), Some(j)) = (i, j) {
        if i < input.len() && j < input[i].len() {
            Some(input[i][j])
        } else {
            None
        }
    } else {
        None
    }
}
