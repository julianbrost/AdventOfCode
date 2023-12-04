#!/usr/bin/env -S awk -f

BEGIN {
	x["one"]   = "1"
	x["two"]   = "2"
	x["three"] = "3"
	x["four"]  = "4"
	x["five"]  = "5"
	x["six"]   = "6"
	x["seven"] = "7"
	x["eight"] = "8"
	x["nine"]  = "9"
	x["1"]     = "1"
	x["2"]     = "2"
	x["3"]     = "3"
	x["4"]     = "4"
	x["5"]     = "5"
	x["6"]     = "6"
	x["7"]     = "7"
	x["8"]     = "8"
	x["9"]     = "9"
	re = "([0-9]|one|two|three|four|five|six|seven|eight|nine)"
}

match($0, /^[^0-9]*([0-9])/, m1) && match($0, /([0-9])[^0-9]*$/, m2) {
	a += m1[1] m2[1];
}

match($0, re ".*", m1) && match($0, ".*" re, m2) {
	b += x[m1[1]] x[m2[1]];
}

END {
	print "Part 1: " a
	print "Part 2: " b
}
