use regex::Regex;
use std::io;

fn main() {
    let input = Regex::new(r"\d+")
        .unwrap()
        .find_iter(io::read_to_string(io::stdin()).unwrap().as_str())
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    assert!(input.len() > 3);
    assert_eq!((input.len() - 3) % 2, 0);

    let res1 = join(&run(input[0], input[1], input[2], &input[3..]));
    println!("Part 1: {}", res1);

    if let Some(res2) = solve(0, &input[3..]) {
        println!("Part 2: {:?}", res2);
    } else {
        // It works with my input, I don't know if your input is similar enough, I doubt there's
        // a generic solution that works for every possible input without knowing additional
        // constraints on the input not stated in the problem description.
        println!("Part 2: That didn't work, good luck with your input!");
    }
}

fn run(mut a: u64, mut b: u64, mut c: u64, code: &[u64]) -> Vec<u64> {
    let mut ip = 0;
    let mut out = Vec::new();
    while ip < code.len() {
        let lop = code[ip + 1];
        let cop = match lop {
            4 => a,
            5 => b,
            6 => c,
            x => x,
        };

        match code[ip] {
            0 /* adv */ => { a = a >> cop; }
            1 /* bxl */ => { b ^= lop; }
            2 /* bst */ => { b = cop % 8; }
            3 /* jnz */ => { if a != 0 { ip = lop as usize; continue; } }
            4 /* bxc */ => { b ^= c; }
            5 /* out */ => { out.push(cop % 8); }
            6 /* bdv */ => { b = a >> cop; }
            7 /* cdv */ => { c = a >> cop; }
            _ => panic!("unknown instruction {}", code[ip]),
        }

        ip += 2;
    }
    out
}

fn join(v: &Vec<u64>) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn solve(a: u64, code: &[u64]) -> Option<u64> {
    for i in 0..8 {
        let next = 8 * a + i;
        let out = run(next, 0, 0, &code);
        if out == code[(code.len() - out.len())..] {
            if out.len() == code.len() {
                return Some(next);
            } else if next != a {
                if let Some(result) = solve(next, code) {
                    return Some(result);
                }
            }
        }
    }
    None
}
