#include <algorithm>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

using namespace std;
using ull = unsigned long long;

int main() {
	vector<pair<ull, ull>> rs;
	string s;

	ull res = 0;
	while (cin >> s) {
		size_t p = s.find('-');
		if (p != string::npos) {
			rs.emplace_back(std::make_pair(stoull(s.substr(0, p)), stoull(s.substr(p+1))));
		} else {
			ull q = stoull(s);
			for (auto [lo, hi] : rs) {
				if (lo <= q && q <= hi) {
					res++;
					break;
				}
			}
		}
	}
	cout << "Part 1: " << res << endl;

	res = 0;
	ull p = 0;
	sort(rs.begin(), rs.end());
	for (auto [lo, hi] : rs) {
		lo = max(p, lo);
		if (lo <= hi) {
			res += hi - lo + 1;
			p = max(p, hi + 1);
		}
	}
	cout << "Part 2: " << res << endl;
}
