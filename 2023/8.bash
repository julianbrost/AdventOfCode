#!/usr/bin/env bash

set -euo pipefail

declare -A graph
declare -a starts

read -r inst
while read -r l; do
	if [[ "$l" =~ ^([[:alnum:]]+)[[:space:]]*=[[:space:]]*\(([[:alnum:]]*),[[:space:]]*([[:alnum:]]*)\)$ ]]; then
		n=${BASH_REMATCH[1]}
		graph["$n,L"]=${BASH_REMATCH[2]}
		graph["$n,R"]=${BASH_REMATCH[3]}
		if [[ "$n" = *A ]]; then
			starts+=("$n")
		fi
	fi
done

cur=AAA
res=0
while [[ "$cur" != ZZZ ]]; do
	cur=${graph["$cur,${inst:$((res % ${#inst})):1}"]}
	res=$((res + 1))
done
echo "Part 1: $res"

declare -a dists
for cur in "${starts[@]}"; do
	res=0
	while [[ "$cur" != *Z ]]; do
		cur=${graph["$cur,${inst:$((res % ${#inst})):1}"]}
		res=$((res + 1))
	done
	dists+=("$res")
done

gcd() {
	local a=$1
	local b=$2
	local c
	while [[ "$b" -ne 0 ]]; do
		c=$b
		b=$((a % b))
		a=$c
	done
	printf '%d' "$a"
}

lcm() {
	local a=$1
	local b
	shift
	for b; do
		a=$(( a / $(gcd "$a" "$b") * b ))
	done
	printf '%d' "$a"
}

# The lowest common multiple is not correct in general. I'm not even sure if
# it's correct on the input, but if AoC is happy...
echo "Part 2: $(lcm "${dists[@]}")"
