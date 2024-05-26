#include <string>
#include <unordered_set>
#include <vector>

using std::string;
using std::unordered_set;
using std::vector;

class Solution {
   public:
    int longestConsecutive(vector<int>& nums) {
        if (nums.empty())
            return 0;

        unordered_set<int> set;
        for (const auto& num : nums)
            set.insert(num);

        int longest = 1;
        for (const auto& num : nums) {
            if (set.count(num - 1))
                continue;

            int curr_long = 1;
            int key = num + 1;
            while (set.count(key)) {
                ++curr_long;
                key += 1;
            }
            if (curr_long > longest)
                longest = curr_long;
        }
        return longest;
    }
};

int main() {
    vector<int> arg = {0, 3, 7, 2, 5, 8, 4, 6, 0, 1};
    Solution s;
    s.longestConsecutive(arg);
    return 0;
}

/** 时间复杂度为 O(n) 的算法解决数字连续的最长序列问题
 * O(n) 所以不能用排序，O(n) 不代表 1n，只要是常数 cn 即可，所以可以多次循环
 * 一遍构造哈希，另一边通过 O(1) 的不断寻找即可，因为只有从最小找才有意义，所以可以剪枝，
 * 这也可以保证绝大部分元素的探寻次数都是 O(1)，所以 c 总体上是常数，所以是 O(n)
 */
