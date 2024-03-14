struct Solution {}
fn main() {}

#[allow(dead_code)]
struct MyStack {}

use std::{cmp::min, collections::HashMap};

#[allow(dead_code)]
impl Solution {
    pub fn integer_replacement_helper(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
        if n == 1 {
            return 0;
        }

        if let Some(&res) = map.get(&n) {
            return res;
        }

        let res = if n % 2 == 0 {
            Solution::integer_replacement_helper(n / 2, map) + 1
        } else {
            2 + min(
                Solution::integer_replacement_helper(n / 2, map),
                Solution::integer_replacement_helper(n / 2 + 1, map),
            )
        };

        map.insert(n, res);
        res
    }

    pub fn integer_replacement(n: i32) -> i32 {
        Solution::integer_replacement_helper(n, &mut HashMap::new())
    }
}

#[test]
fn test() {
    println!("{}", Solution::integer_replacement(2147483647));
    assert_eq!(Solution::integer_replacement(7), 4);
    assert_eq!(Solution::integer_replacement(4), 2);
}
