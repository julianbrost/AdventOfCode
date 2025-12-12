#!/usr/bin/env python3

# This is utterly stupid...

import sys

res1 = 0
shapes = []

for line in sys.stdin:
	line = line.strip()
	if 'x' in line:
		x, y, *cs = map(int, line.replace('x', ' ').replace(':', ' ').split())
		if sum(a*b for a, b in zip(shapes, cs)) <= x*y:
			res1 += 1
	elif ':' in line:
		shapes.append(0)
	else:
		shapes[-1] += line.count('#')

print("Part 1:", res1)
