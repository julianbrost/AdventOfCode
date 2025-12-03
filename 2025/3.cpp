#include <iostream>
#include <string>
#include <vector>

using namespace std;
using ull = unsigned long long;

int main() {
	const ull N = 12;
	ull res1 = 0, res2 = 0;
	string s;
	while (cin >> s) {
		vector<ull> v(N+1);
		for (char c : s) {
			for (ull i = N; i > 0; i--) {
				v[i] = max(v[i], 10 * v[i-1] + c - '0');
			}
		}
		res1 += v[2];
		res2 += v[N];
	}
	cout << "Part 1: " << res1 << endl;
	cout << "Part 2: " << res2 << endl;
}
