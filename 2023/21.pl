#!/usr/bin/env perl

use strict;

chomp(my @map = <STDIN>);
my $N = scalar @map;
my $M = length $map[0];

sub at {
	my $i = shift;
	my $j = shift;

	if (0 <= $i && $i < $N && 0 <= $j && $j < $M) {
		return substr($map[$i], $j, 1);
	} else {
		return '#';
	}
}

my @queue;
my @dist;
push @dist, [(undef) x $M] for 1 .. $N;

for (my $i = 0; $i < $N; $i++) {
	for (my $j = 0; $j < $M; $j++) {
		if (at($i, $j) eq 'S') {
			$dist[$i][$j] = 0;
			push @queue, [$i, $j];
		}
	}
}


while (@queue) {
	my ($i, $j) = @{shift @queue};

	for (my $di = -1; $di <= 1; $di++) {
		for (my $dj = -1; $dj <= 1; $dj++) {
			next if $di * $dj != 0;

			my $ni = $i + $di;
			my $nj = $j + $dj;
			next if at($ni, $nj) eq '#';
			next if defined $dist[$ni][$nj];

			$dist[$ni][$nj] = $dist[$i][$j] + 1;
			push @queue, [$ni, $nj];
		}
	}
}

my $limit = 64; # sample needs 6 here
my $res = 0;

for (my $i = 0; $i < $N; $i++) {
	for (my $j = 0; $j < $M; $j++) {
		my $d = $dist[$i][$j];
		if (defined $d) {
			if ($d <= $limit && $d % 2 == $limit % 2) {
				$res++;
			}
		}
	}
}

print "Part 1: $res\n";
