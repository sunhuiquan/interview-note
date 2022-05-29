#include <bits/stdc++.h>
using namespace std;

// todo: TLE

class Solution
{
public:
	int totalSteps(vector<int> &nums)
	{
		list<int> l;
		for (const auto &e : nums)
			l.push_back(e);
		l.push_back(INT32_MAX);

		int ret = 0;
		bool go = true;
		while (go)
		{
			++ret;
			go = false;

			int old = l.front();
			auto it = l.begin();
			++it;
			// for (; it != l.end();)
			for (; *it != INT32_MAX;)
			{
				if (*it < old)
				{
					go = true;
					old = *it;
					it = l.erase(it);
				}
				else
				{
					old = *it;
					++it;
				}
			}
		}
		return ret - 1;
	}
};

int main()
{
	Solution s;
	vector<int> v{5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11};
	cout << s.totalSteps(v) << endl;

	return 0;
}