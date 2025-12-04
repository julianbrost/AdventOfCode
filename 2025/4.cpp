#include <iostream>
#include <string>
#include <vector>
#include <set>
#include <utility>

using namespace std;

int main() {
	vector<string> input(1);
	string s;
	size_t N = 0, M = 0;
	while (cin >> s) {
		N++;
		M = s.size();
		input.emplace_back(" " + s + " ");
	}
	input[0] = string(M+2, ' ');
	input.emplace_back(input[0]);

	auto movable = [&](size_t i, size_t j) {
		if (input[i][j] != '@') {
			return false;
		}
		size_t o = 0;
		for (size_t di : {-1, 0, 1}) {
			for (size_t dj : {-1, 0, 1}) {
				if (di || dj) {
					if (input[i+di][j+dj] == '@') {
						o++;
					}
				}
			}
		}
		return o < 4;
	};

	set<pair<size_t, size_t>> q;
	for (size_t i = 1; i <= N; i++) {
		for (size_t j = 1; j <= M; j++) {
			if (input[i][j] == '@') {
				if (movable(i, j)) {
					q.insert({i, j});
				}
			}
		}
	}
	cout << "Part 1: " << q.size() << endl;

	size_t res2 = 0;
	while (!q.empty()) {
		auto [i, j] = *q.begin();
		q.erase(q.begin());
		res2++;
		input[i][j] = '.';
		for (size_t di : {-1, 0, 1}) {
			for (size_t dj : {-1, 0, 1}) {
				if (di || dj) {
					if (movable(i+di, j+dj)) {
						q.insert({i+di, j+dj});
					}
				}
			}
		}
	}

	cout << "Part 2: " << res2 << endl;
}
