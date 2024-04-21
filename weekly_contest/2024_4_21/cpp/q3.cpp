#include <map>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using std::min;
using std::pair;
using std::unordered_map;
using std::unordered_set;
using std::vector;

class Solution {
   public:
    int minimumOperations(vector<vector<int>>& grid) {
        int ret = INT32_MAX;
        numrow_ = grid.size();
        if (!numrow_) {
            return 0;
        }
        numcol_ = grid[0].size();

        for (int i = 0; i < numrow_; ++i) {
            ret = min(ret, dfs(grid, 0, i));
        }
        return ret;
    }

   private:
    int dfs(vector<vector<int>>& grid, int col, int row) {
        int idx = row * numcol_ + col;
        if (memory_.find(idx) != memory_.end()) {
            return memory_[idx];
        }

        int current_ops = 0;
        int val = grid[row][col];
        for (int i = 0; i < numrow_; ++i) {
            if (grid[i][col] != val) {
                ++current_ops;
            }
        }

        unordered_set<int> s1;
        unordered_set<int> s2;

        int ops = INT32_MAX;
        if (col + 1 < numcol_) {
            for (int i = 0; i < numrow_; ++i) {
                int v = grid[i][col + 1];
                if (v == val) {
                    continue;
                }

                if (s1.find(v) != s1.end()) {
                    continue;
                }
                s1.insert(v);
                ops = min(ops, dfs(grid, col + 1, i));
            }

            int spec_ops = INT32_MAX;
            if (col + 2 < numcol_) {
                for (int i = 0; i < numrow_; ++i) {
                    int v = grid[i][col + 2];
                    if (s2.find(v) != s2.end()) {
                        continue;
                    }
                    s2.insert(v);
                    spec_ops = min(spec_ops, dfs(grid, col + 2, i));
                }
            }
            if (spec_ops != INT32_MAX) {
                ops = min(ops, spec_ops + numrow_);
            }

            if (ops == INT32_MAX && col + 2 >= numcol_) {
                memory_[idx] = numrow_;
            } else {
                memory_[idx] = ops + current_ops;
            }
        } else {
            memory_[idx] = current_ops;
        }
        return memory_[idx];
    }

    int numrow_;
    int numcol_;
    unordered_map<int, int> memory_;
};

#include <iostream>
using std::cout;
using std::endl;

int main() {
    Solution a;
    vector<vector<int>> input = {{1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}};
    cout << a.minimumOperations(input) << endl;
    return 0;
}
