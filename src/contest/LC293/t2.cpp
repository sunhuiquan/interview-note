#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:

	int maxConsecutive(int bottom, int top, vector<int> &special)
	{
		int ret = 0;
		sort(special.begin(), special.end());

		int a = bottom - 1;
		for (const auto &e : special)
		{
			if (e - a - 1 > ret)
				ret = e - a - 1;
			a = e;
		}
		if (top - a > ret)
			ret = top - a;

		return ret;
	}
};