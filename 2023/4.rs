use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn main() {
	let stdin = io::stdin();
	let mut res1 = 0u64;
	let mut res2 = 0u64;
	let mut dups: VecDeque<u64> = VecDeque::new();
	for line in stdin.lock().lines() {
		if let Some((_game, numbers)) = line.unwrap().split_once(':') {
			if let Some((winstr, gotstr)) = numbers.split_once('|') {
				let win = HashSet::<_>::from_iter(winstr.split_whitespace().map(|x| x.parse::<u64>().unwrap()));
				let got = HashSet::<_>::from_iter(gotstr.split_whitespace().map(|x| x.parse::<u64>().unwrap()));
				let common = got.intersection(&win).count();
				if common > 0 {
					res1 += 2u64.pow((common - 1).try_into().unwrap());
				}
				let n = 1 + dups.pop_front().unwrap_or(0);
				if dups.len() < common {
					dups.resize(common, 0);
				}
				for i in 0..common {
					*dups.get_mut(i).unwrap() += n
				}
				res2 += n;
			}
		}
	}
	println!("Part 1: {}", res1);
	println!("Part 2: {}", res2);
}
