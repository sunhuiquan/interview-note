#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	vector<int> arrayChange(vector<int> &nums, vector<vector<int>> &operations)
	{
		unordered_map<int, int> um;
		int sz = nums.size();
		for (int i = 0; i < sz; ++i)
			um[nums[i]] = i;

		int in;
		for (const auto &v : operations)
		{
			in = um[v[0]];
			um.erase(v[0]);
			um[v[1]] = in;
			nums[in] = v[1];
		}
		return nums;
	}
};

int main()
{
	return 0;
}
