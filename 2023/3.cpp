#include <bits/stdc++.h>

using namespace std;

int main() {
	vector<string> in;
	string line;
	while (cin >> line) {
		in.emplace_back(move(line));
	}

	map<tuple<size_t,size_t>,set<tuple<size_t,size_t,uint64_t>>> neigh;

	uint64_t res = 0;
	for (size_t i = 0; i < in.size(); i++) {
		for (size_t j = 0; j < in[i].size(); j++) {
			if (isdigit(in[i][j])) {
				char *pos = nullptr;
				uint64_t n = strtoll(&in[i][j], &pos, 10);
				size_t len = pos - &in[i][j];

				bool b = false;
				for (size_t k = 0; k < len; k++) {
					for (int di : {-1,0,1}) for (int dj : {-1,0,1}) {
						char c = i+di < in.size() && j+k+dj < in[i+di].size() ? in[i+di][j+k+dj] : '.';

						if (!isdigit(c) && c != '.') {
							b = true;
						}

						if (c == '*') {
							neigh[{i+di, j+k+dj}].insert({i, j, n});
						}
					}
				}

				if (b) {
					res += n;
				}

				j += len;
			}
		}
	}
	cout << "Part 1: " << res << endl;

	res = 0;
	for (const auto &p : neigh) {
		if (p.second.size() == 2) {
			uint64_t n = 1;
			for (const auto &p : p.second) {
				n *= get<2>(p);
			}
			res += n;
		}
	}
	cout << "Part 2: " << res << endl;
}
