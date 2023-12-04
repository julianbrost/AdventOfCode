#!/usr/bin/env python3

import sys
import math
from collections import defaultdict

limits = {
	'red': 12,
	'green': 13,
	'blue': 14,
}

res1 = 0
res2 = 0

for line in sys.stdin:
	game, inp = line.split(':', 1)
	game = int(game.split()[1])
	possible = True
	maxs = defaultdict(int)
	for show in inp.split(';'):
		totals = defaultdict(int)
		for cubes in show.split(','):
			num, color = cubes.split()
			num = int(num)
			totals[color] += num
		for color, limit in limits.items():
			if limit < totals[color]:
				possible = False
		for color, num in totals.items():
			maxs[color] = max(maxs[color], totals[color])
	if possible:
		res1 += game
	res2 += math.prod(maxs.values())

print('Part 1:', res1)
print('Part 2:', res2)
