#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	int rearrangeCharacters(string s, string target)
	{
		unordered_map<char, int> m;
		for (const auto &c : s)
			++m[c];

		unordered_map<char, int> m2;
		for (const auto &c : target)
			++m2[c];

		int ret = INT32_MAX;
		for (const auto &p : m2)
			if (m[p.first] / p.second < ret)
				ret = m[p.first] / p.second;
		if (ret == INT32_MAX)
			return 0;
		return ret;
	}
};
