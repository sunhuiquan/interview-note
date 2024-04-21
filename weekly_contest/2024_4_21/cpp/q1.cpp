#include <string>
#include <unordered_set>
using std::string;
using std::unordered_set;

class Solution {
   public:
    int numberOfSpecialChars(string word) {
        unordered_set<char> m;
        unordered_set<char> res;
        for (const auto &ch : word) {
            m.insert(ch);
            if ((ch >= 'a' && ch <= 'z' && m.find(std::toupper(ch)) != m.end()) ||
                (ch >= 'A' && ch <= 'Z' && m.find(std::tolower(ch)) != m.end())) {
                res.insert(std::tolower(ch));
            }
        }
        return res.size();
    }
};

#include <iostream>
using std::cout;
using std::endl;

int main() {
    Solution a;
    cout << a.numberOfSpecialChars("abBCab") << endl;
    return 0;
}
