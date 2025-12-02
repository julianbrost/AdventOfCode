#!/usr/bin/env python3

import re, itertools, sys

r1, r2 = 0, 0

for lo, hi in itertools.batched(map(int, re.findall(r'\d+', sys.stdin.read())), 2):
	for n in range(lo, hi+1):
		s = str(n)
		l = len(s)
		for i in range(2, l+1):
			if s == s[:l//i] * i:
				r1 += (i == 2) * n
				r2 += n
				break

print('Part 1:', r1)
print('Part 2:', r2)
