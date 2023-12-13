#!/usr/bin/env lua

function read_pattern()
	pattern = {}
	while true do
		local l = io.read()
		if l == nil or l == "" then
			break
		else
			table.insert(pattern, l)
		end
	end
	if #pattern == 0 then
		return nil
	end
	return pattern
end

function count_mismatches(a, b)
	local mismatches = 0
	assert(#a == #b)
	for i = 1, #a do
		if string.sub(a, i, i) ~= string.sub(b, i, i) then
			mismatches = mismatches + 1
		end
	end
	return mismatches
end

local res1 = 0
local res2 = 0
while true do
	pattern = read_pattern()
	if pattern == nil then break end

	for i = 1, #pattern-1 do
		local mismatches = 0
		for j = 0, math.min(i, #pattern-i)-1 do
			mismatches = mismatches + count_mismatches(pattern[i-j], pattern[i+j+1])
		end

		if mismatches == 0 then
			res1 = res1 + 100*i
		elseif mismatches == 1 then
			res2 = res2 + 100*i
		end
	end

	local width = string.len(pattern[1])
	for i = 1, width-1 do
		local mismatches = 0
		local len = math.min(i, width-i)
		for j = 1, #pattern do
			local s1 = string.sub(pattern[j], i-len+1, i)
			local s2 = string.reverse(string.sub(pattern[j], i+1, i+len))
			mismatches = mismatches + count_mismatches(s1, s2)
		end

		if mismatches == 0 then
			res1 = res1 + i
		elseif mismatches == 1 then
			res2 = res2 + i
		end
	end
end

io.write('Part 1: ', res1, '\n')
io.write('Part 2: ', res2, '\n')
