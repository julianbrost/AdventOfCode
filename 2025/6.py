#!/usr/bin/env python3

import sys
import math

nums, ops = [], None
tin = []
for i, line in enumerate(sys.stdin):
	for j, c in enumerate(line.rstrip('\n')):
		if len(tin) == j:
			tin.append("")
		assert len(tin[j]) == i
		tin[j] += c
	line = line.split()
	try:
		nums.append(list(map(int, line)))
	except ValueError:
		ops = line

res = 0
for i, op in enumerate(ops):
	if op == '+':
		res += sum(n[i] for n in nums)
	elif op == '*':
		res += math.prod(n[i] for n in nums)
	else:
		assert False
print("Part 1", res)

res = 0
op, b = None, []
tin.append('')
for i, line in enumerate(tin):
	line = line.strip()
	if line and line[-1] in '+*':
		op = line[-1]
		b.append(int(line[:-1].strip()))
	elif line:
		b.append(int(line))
	else:
		if op == '+':
			res += sum(b)
		elif op == '*':
			res += math.prod(b)
		else:
			assert False
		op, b = None, []
print("Part 2", res)
