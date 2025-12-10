#!/usr/bin/env python3

import sys
from scipy.optimize import milp, LinearConstraint

res2 = 0
for line in sys.stdin:
	indicators, *buttons, joltages = line.split()
	def parse(s):
		return [int(x) for x in s[1:-1].split(',')]
	buttons = list(map(parse, buttons))
	joltages = parse(joltages)
	A = [[int(j in b) for b in buttons] for j in range(len(joltages))]
	r = milp(
		c           = [1]*len(buttons),
		integrality = [1]*len(buttons),
		constraints = [LinearConstraint(A, lb=joltages, ub=joltages)],
	)
	assert r.success
	assert r.fun == int(r.fun)
	res2 += int(r.fun)
print("Part 1: (see C++ implementation)")
print("Part 2:", res2)
