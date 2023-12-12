#!/usr/bin/env kotlin

fun solve(conditions: String, groups: List<Int>, cache: HashMap<Pair<Int,Int>, Long> = hashMapOf()): Long {
	return cache.getOrPut(Pair(conditions.length, groups.size)) {
		if (groups.isEmpty()) {
			return if (conditions.all { it == '.' || it == '?' }) 1 else 0
		}

		val cs = conditions.trimStart('.')
		if (cs.isEmpty()) {
			return if (groups.isEmpty()) 1 else 0
		}
		
		val c = cs[0]
		val g = groups[0]

		var count : Long = 0

		if (cs.length >= g && cs.substring(0, g).all { it == '#' || it == '?' }) {
			if (cs.length == g) {
				count += solve("", groups.drop(1), cache)
			} else if (cs[g] == '.' || cs[g] == '?') {
				count += solve(cs.substring(g+1), groups.drop(1), cache)
			}
		}

		if (c == '?') {
			count += solve(cs.substring(1), groups, cache)
		}

		count
	}
}

var res1 : Long = 0
var res2 : Long = 0

generateSequence(::readLine).forEach {
	val (conditions, groupstr) = it.split(" ")
	val groups = groupstr.split(",").map({ it.toInt() })

	res1 += solve(conditions, groups)
	res2 += solve(List(5) { conditions }.joinToString("?"), List(5) { groups }.flatten())
}

println("Part 1: $res1")
println("Part 2: $res2")
