#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	int minMaxGame(vector<int> &nums)
	{
		int n = nums.size();
		while (n != 1)
		{
			n = n / 2;
			for (int i = 0; i < n; ++i)
			{
				if (i & 1)
					nums[i] = max(nums[2 * i], nums[2 * i + 1]);
				else
					nums[i] = min(nums[2 * i], nums[2 * i + 1]);
			}
		}
		return nums[0];
	}
};

int main()
{
	return 0;
}
