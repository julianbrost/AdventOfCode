use std::io;

fn main() {
    let mut res1 = 0;
    let mut res2 = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (target, args) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let args: Vec<u64> = args.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();

        if possible(target, &args, args.len() - 1, false) {
            res1 += target;
            res2 += target;
        } else if possible(target, &args, args.len() - 1, true) {
            res2 += target;
        }
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn possible(target: u64, args: &Vec<u64>, pos: usize, concat: bool) -> bool {
    let cur = args[pos];

    if pos == 0 {
        return cur == target;
    }

    if target >= cur && possible(target - cur, args, pos - 1, concat) {
        return true;
    }

    if cur > 0 && target % cur == 0 && possible(target / cur, args, pos - 1, concat) {
        return true;
    }

    let f = (0..)
        .map(|e| 10u64.pow(e))
        .filter(|&e| e > cur)
        .next()
        .unwrap();

    if concat
        && target >= cur
        && (target - cur) % f == 0
        && possible((target - cur) / f, args, pos - 1, concat)
    {
        return true;
    }

    false
}
