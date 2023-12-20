#!/usr/bin/env rdmd

module day20;

import std.algorithm : reduce;
import std.algorithm.searching : all;
import std.array : split;
import std.container : DList;
import std.format : format;
import std.numeric : lcm;
import std.stdio : stdin, lines, writeln;
import std.string : strip;
import std.typecons : Tuple, tuple;

struct Input {
	Gate gate;
	size_t input;
}

struct Trigger {
	Input input;
	bool high;
}

class Gate {
	string name;
	Input[] outputTo;

	this(string name) {
		this.name = name;
	}

	Input makeInput() {
		return Input(this, 0);
	}

	void connectTo(Input input) {
		this.outputTo ~= [input];
	}

	Trigger[] receivePulse(size_t input, bool high) {
		Trigger[] t;
		foreach (o; this.outputTo) {
			t ~= Trigger(o, high);
		}
		return t;
	}
}

class FlipFlop : Gate {
	bool on;

	this(string name) {
		super(name);
	}

	override Trigger[] receivePulse(size_t input, bool high) {
		if (!high) {
			this.on = !this.on;

			return super.receivePulse(input, this.on);
		}
		return null;
	}
}

class Conjunction : Gate {
	bool[] inputs;

	this(string name) {
		super(name);
	}

	override Input makeInput() {
		inputs ~= [false];
		return Input(this, inputs.length - 1);
	}

	override Trigger[] receivePulse(size_t input, bool high) {
		this.inputs[input] = high;
		return super.receivePulse(input, !all(this.inputs));
	}
}

void main() {
	Gate[string] gates;
	Tuple!(string,string)[] connections;

	foreach (string line; lines(stdin)) {
		string[] tokens = line.split;
		string name = tokens[0];
		switch (name[0]) {
		case '%':
			name = name[1..$];
			gates[name] = new FlipFlop(name);
			break;

		case '&':
			name = name[1..$];
			gates[name] = new Conjunction(name);
			break;

		default:
			gates[name] = new Gate(name);
		}

		foreach (string output; tokens[2..$]) {
			connections ~= [tuple(name, strip(output, ","))];
		}
	}

	foreach (connection; connections) {
		string from = connection[0];
		string to = connection[1];
		if (!(to in gates)) {
			gates[to] = new Gate(to);
		}
		gates[from].connectTo(gates[to].makeInput());
	}

	string[] rxPred = ["rx"];
	while (rxPred.length == 1 && (rxPred[0] == "rx" || cast(Conjunction) gates[rxPred[0]])) {
		string cur = rxPred[0];
		rxPred = [];
		foreach (g; gates) {
			foreach (o; g.outputTo) {
				if (o.gate.name == cur) {
					rxPred ~= [g.name];
				}
			}
		}
	}
	ulong[string] cycles;
	foreach (pred; rxPred) {
		cycles[pred] = 0;
	}

	Input start = gates["broadcaster"].makeInput();
	ulong[2] pulses;

	for (ulong i = 0; i < 1000 || !all(cycles.byValue); i++) {
		auto queue = DList!Trigger(Trigger(start, false));
		while (!queue.empty()) {
			Trigger t = queue.front();
			queue.removeFront();

			if (i < 1000) {
				pulses[t.high]++;
			}

			if (!t.high && t.input.gate.name in cycles) {
				if (cycles[t.input.gate.name] == 0) {
					cycles[t.input.gate.name] = i + 1;
				}
			}

			Trigger[] ts = t.input.gate.receivePulse(t.input.input, t.high);
			queue ~= ts;
		}
	}

	writeln("Part 1: ", pulses[0] * pulses[1]);
	writeln("Part 2: ", reduce!lcm(ulong(1), cycles));

	// Bonus: circuit in dot format
	if (false) {
		writeln();
		writeln("digraph graphname {");
		foreach (g; gates) {
			foreach (o; g.outputTo) {
				writeln(format(`  "%s" -> "%s";`, g.name, o.gate.name));
			}
		}
		writeln("}");
	}
}
