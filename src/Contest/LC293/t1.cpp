#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	vector<string> removeAnagrams(vector<string> &words)
	{
		vector<pair<string, int>> vp;
		int sz = words.size();
		for (int i = 0; i < sz; ++i)
		{
			string t = words[i];
			sort(t.begin(), t.end());
			vp.push_back({t, i});
		}

		for (int i = 1; i < sz; ++i)
			if (vp[i].first == vp[i - 1].first)
				vp[i].second = -1;

		vector<string> ret;
		for (const auto &t : vp)
			if (t.second != -1)
				ret.push_back(words[t.second]);
		return ret;
	}
};
