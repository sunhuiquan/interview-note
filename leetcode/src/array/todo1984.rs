struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut min_diff = i32::MAX;
        let r = nums.len() - k as usize;
        for i in 0..=r {
            let diff = nums.get(i + k as usize - 1).unwrap() - nums.get(i).unwrap();
            if diff < min_diff {
                min_diff = diff;
            }
        }
        min_diff
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
}
