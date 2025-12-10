#include <bitset>
#include <cassert>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;
using indicators = bitset<16>;

struct Machine {
	indicators expected;
	vector<indicators> buttons;
	vector<unsigned> joltages;
};

int main() {
	string s;
	vector<Machine> input;
	while (cin >> s && !s.empty()) {
		if (s[0] == '[') {
			indicators tmp;
			for (size_t i = 0; i < s.size() && s[1+i] != ']'; i++) {
				if (s[1+i] == '#') {
					tmp[i] = true;
				}
			}
			input.push_back(Machine{tmp, {}, {}});
		} else {
			istringstream str(s);
			char c;
			str >> c;
			if (c == '(') {
				indicators tmp;
				unsigned x;
				while (c != ')' && str >> x >> c) {
					tmp[x] = !tmp[x];
				}
				input.back().buttons.push_back(tmp);
			} else {
				assert(c == '{');
				vector<unsigned> tmp;
				unsigned x;
				while (c != '}' && str >> x >> c) {
					tmp.push_back(x);
				}
				input.back().joltages = move(tmp);
			}
		}
	}

	unsigned res1 = 0;
	for (auto &m : input) {
		size_t p = -1;
		indicators b = 0;
		do {
			indicators r = 0;
			for (size_t i = 0; i < m.buttons.size(); i++) {
				if (b[i]) {
					r ^= m.buttons[i];
				}
			}
			if (m.expected == r) {
				p = min(p, b.count());
			}

			b = b.to_ulong() + 1;
		} while (b.any() && !b[m.buttons.size()]);
		assert(p != unsigned(-1));
		res1 += p;
	}
	cout << "Part 1: " << res1 << endl;
	cout << "Part 2: (see Python implementation)" << endl;
}
