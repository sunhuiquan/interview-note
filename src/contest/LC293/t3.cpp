#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	int largestCombination(vector<int> &candidates)
	{
		sz = candidates.size();
		bt(candidates, 0);
		int ret = 0;
		for (const auto &p : cache)
			if (p.second > ret)
				ret = p.second;
		return ret;
	}

private:
	int sz;
	int ans;
	vector<int> v;
	unordered_map<int, int> cache;

	int bt(const vector<int> &candidates, int i)
	{
		if (cache.count(i))
			return cache[i];

		if (i == sz)
		{
			cache[i] = 0;
			return 0;
		}

		v.push_back(candidates[i]);
		int t = ans;
		if (v.size() == 1)
			ans = candidates[i];
		else
			ans &= candidates[i];

		int ret = 0;
		if (ans > 0)
			ret = bt(candidates, i + 1) + 1;

		ans = t;
		v.pop_back();

		ret = max(ret, bt(candidates, i + 1));
		cache[i] = ret;
		return ret;
	}
};

int main()
{
	Solution s;
	vector<int> v = {84, 40, 66, 44, 91, 90, 1, 14, 73, 51, 47, 35, 18, 46, 18, 65, 55, 18, 16, 45, 43, 58, 90, 92, 91, 43, 44, 76, 85, 72, 24, 89, 60, 94, 81, 90, 86, 79, 84, 41, 41, 28, 44};
	cout << s.largestCombination(v) << endl;

	return 0;
}