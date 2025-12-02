#include <iostream>
#include <unordered_set>

using namespace std;
using ull = unsigned long long;

int main() {
	ull res1 = 0, res2 = 0;
	char c;
	do {
		ull lo, hi;
		cin >> lo >> c >> hi;
		unordered_set<ull> u;
		for (ull pattern = 1; pattern * pattern <= hi; pattern++) {
			ull x = pattern;
			bool first = true;
			do {
				for (ull n = pattern; n > 0; n /= 10) {
					x *= 10;
				}
				x += pattern;
				if (lo <= x && x <= hi) {
					if (first) res1 += x;
					if (u.insert(x).second) res2 += x;
				}
				first = false;
			} while (x < hi);
		}
	} while (cin >> c && c == ',');
	cout << "Part 1: " << res1 << endl;
	cout << "Part 2: " << res2 << endl;
}
