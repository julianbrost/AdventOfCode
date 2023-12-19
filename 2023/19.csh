#!/usr/bin/env tcsh

set workflows = ()

while (1)
	set workflow = $<:q
	if ($workflow:q == '') then
		break
	endif
	set workflows = ($workflows:q $workflow:s/\{/ /:s/\}//:as/,/ /:q)
end

set res = 0

while (1)
	set part = "$<"
	if ($part:q == '') then
		break
	endif
	set part = ($part:s/\{//:s/\}//:as/,/ /)

	set workflow = in
	while ($workflow != 'A' && $workflow != 'R')
		foreach w ($workflows:q)
			set w = ($w)
			if ($w[1] == $workflow) then
				set workflow = ($w)
				break
			endif
		end
		shift workflow
		foreach rule ($workflow:q)
			set rule = ($rule:s/:/ /)
			if ($#rule == 1) then
				set workflow = $rule[1]
				break
			else
				set lt = ($rule[1]:s/</ /)
				set gt = ($rule[1]:s/>/ /)
				if ($#lt == 2) then
					set var = $lt[1]
				else if ($#gt == 2) then
					set var = $gt[1]
				endif
				foreach kv ($part:q)
					set kv = ($kv:s/=/ /)
					if ($kv[1] == $var) then
						set val = $kv[2]
					endif
				end
				if ($#lt == 2) then
					if ($val < $lt[2]) then
						set workflow = $rule[2]
						break
					endif
				else if ($#gt == 2) then
					if ($val > $gt[2]) then
						set workflow = $rule[2]
						break
					endif
				endif
			endif
		end
	end

	if ($workflow == 'A') then
		foreach kv ($part:q)
			set kv = ($kv:s/=/ /)
			@ res = ($res + $kv[2])
		end
	endif
end

echo "Part 1: $res"

set res = 0
set queue = (in)

while ($#queue > 0)
	set conds = ($queue[1])
	shift queue
	set name = $conds[1]
	shift conds

	if ($name == 'A') then
		set xlo = 1
		set mlo = 1
		set alo = 1
		set slo = 1
		set xhi = 4000
		set mhi = 4000
		set ahi = 4000
		set shi = 4000
		foreach cond ($conds:q)
			set lt = ($cond:s/</ /)
			set gt = ($cond:s/>/ /)
			if ($#lt == 2) then
				if ($lt[1] == 'x' && $xhi >= $lt[2]) then
					@ xhi = ($lt[2] - 1)
				else if ($lt[1] == 'm' && $mhi >= $lt[2]) then
					@ mhi = ($lt[2] - 1)
				else if ($lt[1] == 'a' && $ahi >= $lt[2]) then
					@ ahi = ($lt[2] - 1)
				else if ($lt[1] == 's' && $shi >= $lt[2]) then
					@ shi = ($lt[2] - 1)
				endif
			else if ($#gt == 2) then
				if ($gt[1] == 'x' && $xlo <= $gt[2]) then
					@ xlo = ($gt[2] + 1)
				else if ($gt[1] == 'm' && $mlo <= $gt[2]) then
					@ mlo = ($gt[2] + 1)
				else if ($gt[1] == 'a' && $alo <= $gt[2]) then
					@ alo = ($gt[2] + 1)
				else if ($gt[1] == 's' && $slo <= $gt[2]) then
					@ slo = ($gt[2] + 1)
				endif
			endif
		end
		@ res = ($res + ($xhi - $xlo + 1) * ($mhi - $mlo + 1) * ($ahi - $alo + 1) * ($shi - $slo + 1))
		continue
	else if ($name == 'R') then
		continue
	endif

	foreach w ($workflows:q)
		set w = ($w)
		if ($w[1] == $name) then
			set rules = ($w)
			shift rules
			break
		endif
	end

	foreach rule ($rules:q)
		set rule = ($rule:s/:/ /)
		if ($#rule == 1) then
			set queue = ($queue:q "$rule[1] $conds")
			break
		else
			set queue = ($queue:q "$rule[2] $conds $rule[1]")
			set lt = ($rule[1]:s/</ /)
			set gt = ($rule[1]:s/>/ /)
			if ($#lt == 2) then
				@ val = ($lt[2] - 1)
				set neg = "$lt[1]>$val"
			else if ($#gt == 2) then
				@ val = ($gt[2] + 1)
				set neg = "$gt[1]<$val"
			endif
			set conds = ($conds "$neg")
		endif
	end
end

echo "Part 2: $res"
