use std::collections::HashMap;

struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut l_map = HashMap::new();
        let mut r_map = HashMap::new();
        let mut fre_map = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            l_map.entry(num).or_insert(idx);
            r_map.insert(num, idx);
            let count = fre_map.entry(num).or_insert(0);
            *count += 1;
        }

        let mut max_fre = 0;
        let mut min_degree = usize::MAX;
        for (&num, fre) in fre_map.iter() {
            let degree = r_map.get(num).unwrap() - l_map.get(num).unwrap() + 1;
            if *fre > max_fre {
                max_fre = *fre;
                min_degree = degree;
            } else if *fre == max_fre && degree < min_degree {
                min_degree = degree;
            }
        }
        min_degree as i32
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    assert_eq!(
        Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
        6
    );
}
