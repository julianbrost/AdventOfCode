#!/usr/bin/env ruby

map = STDIN.readlines

neigh = {
	'|' => [[-1,0], [1,0]],
	'-' => [[0,-1], [0,1]],
	'L' => [[-1,0], [0,1]],
	'J' => [[-1,0], [0,-1]],
	'7' => [[0,-1], [1,0]],
	'F' => [[0,1], [1,0]],
	'S' => [[-1,0], [1,0], [0,-1], [0,1]],
}

maxi = map.length
maxj = 0
starti = startj = -1
map.length.times do |i|
	map[i].strip!
	maxj = [maxj, map[i].length].max
	map[i].length.times do |j|
		if map[i][j] == 'S' then
			starti, startj = i, j
		end
	end
end

dist = Array.new(2*maxi) { Array.new(2*maxj) }

dist[2*starti][2*startj] = 0
q = Array.new
q.push [starti, startj]

maxd = 0
while not q.empty? do
	i, j = q.shift
	maxd = [maxd, dist[2*i][2*j]].max

	n = neigh[map[i][j]]
	next if n.nil?

	n.each do |(di,dj)|
		ni, nj = i+di, j+dj

		next unless (0..maxi-1).include?(ni)
		next unless (0..maxj-1).include?(nj)
		next unless neigh[map[ni][nj]].include?([-di,-dj])

		dist[2*i+di][2*j+dj] = dist[2*i][2*j]

		next unless dist[2*ni][2*nj].nil?

		dist[2*ni][2*nj] = dist[2*i][2*j] + 1

		q.push [ni, nj]
	end
end

q = Array.new
def enqueue_unvisited(q, dist, i, j)
	return unless (0..dist.length-1).include?(i)
	return unless (0..dist[i].length-1).include?(j)
	return unless dist[i][j].nil?
	dist[i][j] = -1
	q.push [i,j]
end
(2*maxi).times do |i|
	enqueue_unvisited(q, dist, i, 0)
	enqueue_unvisited(q, dist, i, 2*maxj-1)
end
(2*maxj).times do |j|
	enqueue_unvisited(q, dist, 0, j)
	enqueue_unvisited(q, dist, 2*maxi-1, j)
end

while not q.empty? do
	i, j = q.shift
	enqueue_unvisited(q, dist, i+1, j)
	enqueue_unvisited(q, dist, i-1, j)
	enqueue_unvisited(q, dist, i, j+1)
	enqueue_unvisited(q, dist, i, j-1)
end

inside = 0
maxi.times do |i|
	maxj.times do |j|
		if dist[2*i][2*j].nil? then
			inside += 1
		end
	end
end

pretty = {
	'S' => '╬',
	'L' => '╚',
	'J' => '╝',
	'7' => '╗',
	'F' => '╔',
	'-' => '═',
	'|' => '║',
	'.' => ' ',
}

(2*maxi).times do |i|
	(2*maxj).times do |j|
		c = '╬'
		if i.even? && j.even? then
			c = pretty[map[i/2][j/2]]
		elsif dist[i][j].nil? || dist[i][j] == -1
			c = '·'
		end
		if dist[i][j].nil? then
			print c
		elsif dist[i][j] == 0 then
			print "\e[31m#{c}\e[0m"
		elsif dist[i][j] == -1 then
			print "\e[32m#{c}\e[0m"
		else
			print "\e[34m#{c}\e[0m"
		end
	end
	puts
end

puts
puts "Part 1: #{maxd}"
puts "Part 2: #{inside}"
