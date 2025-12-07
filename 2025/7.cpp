#include <iostream>
#include <string>
#include <vector>

using namespace std;
using ull = unsigned long long;

int main() {
	string s;
	vector<ull> p;
	ull res1 = 0, res2 = 0;
	while (cin >> s) {
		p.resize(s.size());
		vector<ull> n(s.size());
		for (size_t i = 0; i < s.size(); i++) {
			if (s[i] == 'S') {
				n[i]++;
			} else if (s[i] == '^' && p[i]) {
				res1++;
				if (i > 0) {
					n[i-1] += p[i];
				}
				if (i < n.size() - 1) {
					n[i+1] += p[i];
				}
			} else {
				n[i] += p[i];
			}
		}
		p = move(n);
	}
	for (ull i : p) {
		res2 += i;
	}
	cout << "Part 1: " << res1 << endl;
	cout << "Part 2: " << res2 << endl;
}
