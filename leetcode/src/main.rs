use std::collections::HashMap;

struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_freq = 0;
        let mut min_sub_len = i32::MAX;

        let mut first_occur: HashMap<i32, i32> = HashMap::new();
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let freq = freq_map.get_mut(&num).unwrap();
            *freq += 1;
            if *freq > max_freq {
                max_freq = *freq;
            }
        }

        -1
    }
}

#[test]
fn test1() {
    // assert_eq!(
    //     Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
    //     vec![vec![1, 2, 3, 4]]
    // );
}
