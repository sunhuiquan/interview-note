#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	int partitionArray(vector<int> &nums, int k)
	{
		int sz = nums.size();
		sort(nums.begin(), nums.end());
		int l = 0, ret = 0;
		for (int r = 1; r < sz; ++r)
		{
			if (nums[r] - nums[l] > k)
			{
				++ret;
				l = r;
			}
		}
		return ret + 1;
	}
};

int main()
{
	return 0;
}
