#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	int minPathCost(vector<vector<int>> &grid, vector<vector<int>> &moveCost)
	{
		m = grid.size();
		if (grid.empty())
			return -1;
		n = grid[0].size();

		cache.resize(m);
		for (int i = 0; i < m; ++i)
			cache[i].resize(n + 1);

		for (int i = 0; i < m; ++i)
			for (int j = 0; j < n; ++j)
			{
				cache[i][j].resize(n);
				for (int k = 0; k < n; ++k)
					cache[i][j][k] = -1;
			}

		for (int i = 0; i < n; ++i)
		{
			int ans = 0;
			br(grid, moveCost, 0, 0, i, ans);
		}

		int ret = INT32_MAX;
		for (int i = 0; i < n; ++i)
			if (cache[0][i][0] < ret)
				ret = cache[0][i][0];
		return ret;
	}

private:
	int m, n;
	int ans = 0;
	vector<vector<vector<int>>> cache;

	int br(const vector<vector<int>> &grid, const vector<vector<int>> &moveCost, int l, int from, int in, int &ans)
	{
		if (cache[l][in][from] != -1)
			return cache[l][in][from];

		if (l == m - 1)
		{
			cache[l][in][from] = ans + grid[l][in];
			return cache[l][in][from];
		}

		int t = ans;
		int ret = INT32_MAX, a;
		for (int i = 0; i < n; ++i)
		{
			ans += moveCost[grid[l][in]][i] + grid[l][in];
			a = br(grid, moveCost, l + 1, in, i, ans);
			cache[l][in][from] = a;
			if (a < ret)
				ret = a;
			ans = t;
		}
		return ret;
	}
};

int main()
{
	Solution s;
	vector<vector<int>> a{{5, 3}, {4, 0}, {2, 1}};
	vector<vector<int>> b{{9, 8}, {1, 5}, {10, 12}, {18, 6}, {2, 4}, {14, 3}};
	cout << s.minPathCost(a, b) << endl;
}
