#!/usr/bin/env php
<?php

class Hand {
	public $cards;
	public $bid;

	public function __construct($cards, $bid) {
		$this->cards = $cards;
		$this->bid = $bid;
	}

	public function value1() {
		$type_values = [[5], [4,1], [3,2], [3,1,1], [2,2,1], [2,1,1,1], [1,1,1,1,1]];
		$card_values = "AKQJT98765432";

		$value = array();

		$freqs = array_values(array_count_values(str_split($this->cards)));
		rsort($freqs);
		$value[] = array_search($freqs, $type_values);

		foreach (str_split($this->cards) as $card) {
			$value[] = strpos($card_values, $card);
		}

		return $value;
	}

	public function value2() {
		$type_values = [[5], [4,1], [3,2], [3,1,1], [2,2,1], [2,1,1,1], [1,1,1,1,1]];
		$card_values = "AKQT98765432J";

		$value = array();

		$freq_by_card = array_count_values(str_split($this->cards));
		$jokers = 0;
		if (isset($freq_by_card['J'])) {
			$jokers = $freq_by_card['J'];
			unset($freq_by_card['J']);
		}
		$freqs = array_values($freq_by_card);
		rsort($freqs);
		if (empty($freqs)) {
			$freqs[0] = $jokers;
		} else {
			$freqs[0] += $jokers;
		}
		$value[] = array_search($freqs, $type_values);

		foreach (str_split($this->cards) as $card) {
			$value[] = strpos($card_values, $card);
		}

		return $value;
	}
}

$hands = array();
while (fscanf(STDIN, "%s %d\n", $hand, $bid)) {
	$hands[] = new Hand($hand, $bid);
}

usort($hands, function($a, $b) {
	return -($a->value1() <=> $b->value1());
});
$res = 0;
foreach ($hands as $index => $hand) {
	$res += ($index + 1) * $hand->bid;
}
echo "Part 1: $res\n";

usort($hands, function($a, $b) {
	return -($a->value2() <=> $b->value2());
});
$res = 0;
foreach ($hands as $index => $hand) {
	$res += ($index + 1) * $hand->bid;
}
echo "Part 2: $res\n";
