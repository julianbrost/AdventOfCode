use std::io;

fn main() {
    let input: Vec<u8> = io::read_to_string(io::stdin())
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut res = 0;
    let mut block = 0;
    let mut lo = 0;
    let mut hi = input.len();
    let mut moved: Vec<u8> = vec![0; input.len()];
    while lo < hi {
        if lo % 2 == 0 {
            for _i in 0..(input[lo] - moved[lo]) {
                res += block * (lo / 2);
                block += 1;
            }
        } else {
            for _i in 0..(input[lo] - moved[lo]) {
                if input[hi - 1] == moved[hi - 1] {
                    hi -= 2;
                    if lo >= hi {
                        break;
                    }
                }
                res += block * (hi / 2);
                block += 1;
                moved[hi - 1] += 1;
            }
        }
        lo += 1;
    }
    println!("Part 1: {}", res);

    // The complexity of part 2 isn't ideal, using a linked list with a run-length encoding would
    // probably be way faster, but after reading into linked lists in Rust, I lost interest.
    let mut files: Vec<Option<usize>> = Vec::new();
    for (i, &s) in input.iter().enumerate() {
        let id = if i % 2 == 0 { Some(i / 2) } else { None };
        for _ in 0..s {
            files.push(id);
        }
    }
    for id in (0..(input.len() / 2 + 1)).rev() {
        let size = input[2 * id];
        let mut free = 0;
        for (i, &f) in files.iter().enumerate() {
            if f == Some(id) {
                break;
            } else if f == None {
                free += 1;
                if free == size {
                    for j in (i - free as usize + 1)..=i {
                        files[j] = Some(id)
                    }
                    for j in (i + 1)..files.len() {
                        if files[j] == Some(id) {
                            files[j] = None;
                        }
                    }
                    break;
                }
            } else {
                free = 0;
            }
        }
    }
    let mut res = 0;
    for (i, &f) in files.iter().enumerate() {
        if let Some(id) = f {
            res += i * id
        }
    }
    println!("Part 2: {}", res);
}
