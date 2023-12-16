#!/usr/bin/env julia

const left = 1
const right = 2
const up = 3
const down = 4

map = readlines()
N = length(map)
M = length(map[1])

function qpush(q, v, i, j, d)
	j -= d == left
	j += d == right
	i -= d == up
	i += d == down
	if 1 <= i <= N && 1 <= j <= M && !v[i,j,d]
		push!(q, (i, j, d))
		v[i,j,d] = true
	end
end

function count(i, j, d)
	queue = Set{Tuple{Int,Int,Int}}()
	visited = falses(N, M, 4)

	push!(queue, (i, j, d))
	visited[i,j,d] = true

	while !isempty(queue)
		c = first(queue)
		pop!(queue, c)
		(i, j, d) = c
		v = map[i][j]
		if v == '|' && (d == left || d == right)
			qpush(queue, visited, i, j, up)
			qpush(queue, visited, i, j, down)
		elseif v == '-' && (d == up || d == down)
			qpush(queue, visited, i, j, left)
			qpush(queue, visited, i, j, right)
		elseif v == '/' && d == left
			qpush(queue, visited, i, j, down)
		elseif v == '/' && d == right
			qpush(queue, visited, i, j, up)
		elseif v == '/' && d == up
			qpush(queue, visited, i, j, right)
		elseif v == '/' && d == down
			qpush(queue, visited, i, j, left)
		elseif v == '\\' && d == left
			qpush(queue, visited, i, j, up)
		elseif v == '\\' && d == right
			qpush(queue, visited, i, j, down)
		elseif v == '\\' && d == up
			qpush(queue, visited, i, j, left)
		elseif v == '\\' && d == down
			qpush(queue, visited, i, j, right)
		else
			qpush(queue, visited, i, j, d)
		end
	end

	res = 0
	for i in 1:N
		for j in 1:M
			res += any(visited[i,j,:])
		end
	end
	return res
end

res = count(1, 1, right)
print("Part 1: $res\n")

for i = 1:N
	global res = max(res, count(i, 1, right), count(i, M, left))
end
for j = 1:M
	global res = max(res, count(1, j, down), count(N, j, up))
end
print("Part 2: $res\n")
