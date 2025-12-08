#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>

using namespace std;
using ll = long long;

struct UnionFind {
	vector<size_t> p;

	UnionFind(size_t n) : p(n) {
		for (size_t i = 0; i < n; i++) p[i] = i;
	}

	size_t Find(size_t x) {
		if (p[x] != x) p[x] = Find(p[x]);
		return p[x];
	}

	bool Union(size_t x, size_t y) {
		x = Find(x);
		y = Find(y);
		if (x == y) {
			return false;
		} else {
			p[x] = y;
			return true;
		}
	}
};

int main() {
	vector<tuple<ll,ll,ll>> ps;
	{
		ll x, y, z;
		char c;
		while (cin >> x >> c >> y >> c >> z) {
			ps.push_back({x, y, z});
		}
	}
	vector<tuple<ll,ll,ll>> ds;
	for (size_t i = 0; i < ps.size(); i++) {
		for (size_t j = i+1; j < ps.size(); j++) {
			auto [xi, yi, zi] = ps[i];
			auto [xj, yj, zj] = ps[j];
			ll dx = xi - xj, dy = yi - yj, dz = zi - zj;
			ll d = dx*dx + dy*dy + dz*dz;
			ds.push_back({d, i, j});
		}
	}
	UnionFind u(ps.size());
	ll res2 = 0;
	sort(ds.begin(), ds.end());
	for (size_t i = 0; i < ds.size(); i++) {
		auto [d, a, b] = ds[i];
		if (u.Union(a, b)) {
			res2 = get<0>(ps[a]) * get<0>(ps[b]);
		}
		if (i == 9 || i == 999) {
			vector<size_t> sz(ps.size());
			for (size_t i = 0; i < ps.size(); i++) {
				sz[u.Find(i)]++;
			}
			sort(sz.begin(), sz.end(), greater());
			ll res1 = sz[0] * sz[1] * sz[2];
			cout << "Part 1: " << res1 << " (after " << (i+1) <<" steps)" << endl;
		}
	}
	cout << "Part 2: " << res2 << endl;
}
