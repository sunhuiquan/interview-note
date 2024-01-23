struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;
        for e in nums {
            count = if e != 1 { 0 } else { count + 1 };
            if count > max {
                max = count;
            };
        }
        max
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    )
}
