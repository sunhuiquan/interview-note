#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	double calculateTax(vector<vector<int>> &brackets, int income)
	{
		double ret = 0;
		bool is_first = true;
		int sz = brackets.size();

		for (int i = 0; i < sz; ++i)
		{
			double rate;
			if (!brackets[i][1])
				rate = 0;
			else
				rate = brackets[i][1] / 100.0;

			if (income > brackets[i][0])
			{
				if (is_first)
				{
					ret += brackets[i][0] * rate;
					is_first = false;
				}
				else
					ret += (brackets[i][0] - brackets[i - 1][0]) * rate;
			}
			else
			{
				if (is_first)
				{
					ret += income * rate;
					is_first = false;
				}
				else
					ret += (income - brackets[i - 1][0]) * rate;
				break;
			}
		}

		return ret;
	}
};

int main()
{
	// 	[[1,0],[2,97],[3,20],[4,89],[5,54]]
	// 5
	Solution s;
	vector<vector<int>> v{{1, 0}, {2, 97}, {3, 20}, {4, 89}, {5, 54}};
	cout << s.calculateTax(v, 5) << endl;

	return 0;
}
