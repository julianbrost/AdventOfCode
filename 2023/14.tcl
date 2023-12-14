#!/usr/bin/env tclsh

while {[gets stdin line] >= 0} {
	lappend map $line
}

proc tilt {map} {
	set height [llength $map]
	set width [string length [lindex $map 0]]

	set up [lrepeat $width 0]

	for {set i 0} {$i < $height} {incr i} {
		set row [lindex $map $i]
		while true {
			set j [string first ".O" $row]
			if {$j < 0} {
				break
			}
			set row [string replace $row $j [expr {$j + 1}] "O."]
		}
		lset map $i $row
	}

	return $map
}

proc rotate {map} {
	set height [llength $map]
	set width [string length [lindex $map 0]]

	for {set i 0} {$i < $width} {incr i} {
		set row ""
		for {set j 0} {$j < $height} {incr j} {
			set row [string cat $row [string index [lindex $map [expr {$height - $j - 1}]] $i]]
		}
		lappend rotated $row
	}

	return $rotated
}

proc weight {map} {
	set height [llength $map]
	set width [string length [lindex $map 0]]

	set w 0

	for {set i 0} {$i < $height} {incr i} {
		for {set j 0} {$j < $width} {incr j} {
			if {[string index [lindex $map $i] $j] == "O"} {
				set w [expr {$w + $width - $j}]
			}
		}
	}

	return $w
}

set map [tilt [rotate [rotate [rotate $map]]]]
puts "Part 1: [weight $map]"

for {set rem [expr {1000000000 - 1}]} {$rem >= 0} {incr rem -1} {
	set map [rotate [tilt $map]]
	set map [rotate [tilt $map]]
	set map [rotate [tilt $map]]
	set map [rotate [tilt $map]]
	if {[info exists cache($map)]} {
		set rem [expr {$rem % ($cache($map) - $rem)}]
	} else {
		set cache($map) $rem
	}
}
puts "Part 2: [weight $map]"
