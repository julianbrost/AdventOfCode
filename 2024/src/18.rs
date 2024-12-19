use std::collections::VecDeque;
use std::io;

const N: usize = 71;

fn main() {
    let mut map = vec![vec![true; N]; N];
    for (i, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let (x, y) = line.split_once(",").unwrap();
        map[x.parse::<usize>().unwrap()][y.parse::<usize>().unwrap()] = false;

        let mut dist: Vec<Vec<Option<u64>>> = vec![vec![None; N]; N];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        dist[0][0] = Some(0);
        queue.push_back((0, 0));

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if (x > 0 || dx > 0) && (y > 0 || dy > 0) {
                    let nx = x.checked_add_signed(dx).unwrap();
                    let ny = y.checked_add_signed(dy).unwrap();
                    if nx < N && ny < N && map[nx][ny] && dist[nx][ny].is_none() {
                        dist[nx][ny] = Some(dist[x][y].unwrap() + 1);
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        if i == 1023 {
            println!("Part 1: {}", dist.last().unwrap().last().unwrap().unwrap());
        }
        if dist.last().unwrap().last().unwrap().is_none() {
            println!("Part 2: {},{}", x, y);
            break;
        }
    }
}
