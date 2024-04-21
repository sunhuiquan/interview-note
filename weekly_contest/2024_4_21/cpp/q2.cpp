#include <string>
#include <unordered_map>
#include <unordered_set>
using std::string;
using std::unordered_map;
using std::unordered_set;

class Solution {
   public:
    int numberOfSpecialChars(string word) {
        unordered_set<char> s;
        unordered_map<char, int> res;
        for (const auto& ch : word) {
            if (ch >= 'a' && ch <= 'z') {
                s.insert(ch);
            }

            if (ch >= 'A' && ch <= 'Z') {
                if (s.find(std::tolower(ch)) != s.end()) {
                    if (res.find(std::tolower(ch)) == res.end()) {
                        res[std::tolower(ch)] = 1;
                    }
                } else {
                    res[std::tolower(ch)] = -1;
                }
            }

            if (ch >= 'a' && ch <= 'z' && res.find(ch) != res.end()) {
                res[ch] = -1;
            }
        }

        int ret = 0;
        for (const auto& p : res) {
            if (p.second == 1) {
                ++ret;
            }
        }
        return ret;
    }
};

#include <iostream>
using std::cout;
using std::endl;

int main() {
    Solution a;
    cout << a.numberOfSpecialChars("AbcbDBdD") << endl;
    return 0;
}
