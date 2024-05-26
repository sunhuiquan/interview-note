#include <algorithm>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
   public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, vector<string>> map;
        for (const auto& str : strs) {
            string tmp = str;
            std::sort(tmp.begin(), tmp.end());
            map[tmp].push_back(str);
        }

        vector<vector<string>> ans;
        for (const auto& iter : map) {
            ans.push_back(iter.second);
        }
        return ans;
    }
};

int main() {
    vector<string> arg = {"eat", "tea", "tan", "ate", "nat", "bat"};
    Solution s;
    s.groupAnagrams(arg);
    return 0;
}

/** 字母异位词
 * 将字母异位词排序，互为异位的单词能够得到相同的键，可以用哈希。
 */
