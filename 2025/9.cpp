#include <algorithm>
#include <cmath>
#include <iostream>
#include <queue>
#include <utility>
#include <vector>

using namespace std;
using ll = long long;

int main() {
	vector<pair<ll,ll>> p;
	vector<ll> xs, ys;
	ll x, y;
	char c;
	while (cin >> x >> c >> y) {
		p.push_back({x, y});
		for (ll d : {-1, 0, 1}) {
			xs.push_back(x + d);
			ys.push_back(y + d);
		}
	}

	sort(xs.begin(), xs.end());
	xs.erase(unique(xs.begin(), xs.end()), xs.end());

	sort(ys.begin(), ys.end());
	ys.erase(unique(ys.begin(), ys.end()), ys.end());

	auto comp = [&xs, &ys](pair<ll,ll> xy) -> pair<ll,ll> {
		auto [x, y] = xy;
		return make_pair(
			lower_bound(xs.begin(), xs.end(), x) - xs.begin(),
			lower_bound(ys.begin(), ys.end(), y) - ys.begin()
		);
	};

	vector<vector<char>> m(xs.size()+2, vector<char>(ys.size()+2));
	for (size_t i = 0; i < p.size(); i++) {
		auto [xi, yi] = comp(p[i]);
		auto [xj, yj] = comp(p[(i+1)%p.size()]);

		m[xi][yi] = '#';
		m[xj][yj] = '#';

		ll dx = xj - xi;
		if (dx) dx /= abs(dx);
		ll dy = yj - yi;
		if (dy) dy /= abs(dy);

		while (xi != xj || yi != yj) {
			m[xi][yi] = '#';
			xi += dx;
			yi += dy;
		}
	}

	queue<pair<size_t,size_t>> q;
	q.push({xs.size()+1, ys.size()+1});
	m[xs.size()+1][ys.size()+1] = '.';
	while (!q.empty()) {
		auto [x, y] = q.front();
		q.pop();
		for (ll dx : {-1, 0, 1}) for (ll dy : {-1, 0, 1}) {
			ll xx = x + dx, yy = y + dy;
			if (
				0 <= xx && size_t(xx) < xs.size()+2 &&
				0 <= yy && size_t(yy) < ys.size()+2 &&
				m[xx][yy] == 0
			) {
				q.push({xx, yy});
				m[xx][yy] = '.';
			}
		}
	}
	for (auto &l : m) {
		for (char &c : l) {
			if (c == 0) {
				c = 'X';
			}
		}
	}

	ll res1 = 0, res2 = 0;
	for (size_t i = 0; i < p.size(); i++) {
		for (size_t j = i+1; j < p.size(); j++) {
			auto [xi, yi] = p[i];
			auto [xj, yj] = p[j];
			auto [cxi, cyi] = comp(p[i]);
			auto [cxj, cyj] = comp(p[j]);

			ll a = (abs(xi - xj) + 1) * (abs(yi - yj) + 1);
			res1 = max(res1, a);

			ll dx = cxj - cxi;
			if (dx) dx /= abs(dx);
			ll dy = cyj - cyi;
			if (dy) dy /= abs(dy);
			bool valid = true;
			for (ll cx = cxi; valid && cx != cxj; cx += dx) {
				for (ll cy = cyi; valid && cy != cyj; cy += dy) {
					if (m[cx][cy] == '.') {
						valid = false;
					}
				}
			}
			if (valid) {
				res2 = max(res2, a);
			}
		}
	}

	cout << "Part 1: " << res1 << endl;
	cout << "Part 2: " << res2 << endl;
}
