#include <iostream>

using namespace std;

int main() {
	int res1 = 0, res2 = 0;
	int x = 50;
	char c; int n;
	while (cin >> c >> n) {
		if (c == 'L') {
			if (x > 0) x -= 100;
			x -= n;
			res2 -= x / 100;
		} else {
			if (x < 0) x += 100;
			x += n;
			res2 += x / 100;
		}
		x %= 100;
		res1 += (x == 0);
	}
	cout << "Part 1: " << res1 << endl;
	cout << "Part 2: " << res2 << endl;
}
