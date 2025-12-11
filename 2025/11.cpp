#include <iostream>
#include <map>
#include <queue>
#include <string>
#include <tuple>
#include <unordered_map>
#include <utility>
#include <vector>

using namespace std;
using ull = unsigned long long;

int main() {
	string s, cur;
	unordered_map<string, vector<string>> adj;
	unordered_map<string, size_t> indeg;
	while (cin >> s) {
		if (s.back() == ':') {
			s.pop_back();
			indeg[s]; // this is cursed: ensure s exists as key
			cur = move(s);
		} else {
			indeg[s]++;
			adj[cur].emplace_back(move(s));
		}
	}

	queue<string> q;
	for (const auto& [n, i] : indeg) {
		if (i == 0) {
			q.push(n);
		}
	}

	vector<string> ts;
	while (!q.empty()) {
		string c = move(q.front());
		q.pop();
		for (const string& n : adj[c]) {
			if (--indeg[n] == 0) {
				q.push(n);
			}
		}
		ts.emplace_back(move(c));
	}

	unordered_map<string,ull> p1 {{"you", 1}};
	for (auto &c : ts) {
		for (const string &n : adj[c]) {
			p1[n] += p1[c];
		}
	}
	cout << "Part 1: " << p1["out"] << endl;

	map<tuple<string,bool,bool>,ull> p2 {{{"svr", false, false}, 1}};
	for (auto &c : ts) {
		for (const string &n : adj[c]) {
			for (bool dac : {false, true}) {
				for (bool fft : {false, true}) {
					p2[{n, dac || n == "dac", fft || n == "fft"}] += p2[{c, dac, fft}];
				}
			}
		}
	}
	cout << "Part 1: " << p2[{"out", true, true}] << endl;
}
