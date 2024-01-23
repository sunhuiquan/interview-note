use std::vec;

struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        const FLAG_NUM: i32 = 100000;
        let mut nums = nums;
        let size = nums.len();

        for i in 0..size {
            let e = *nums.get(i).unwrap();
            let index = if e >= FLAG_NUM {
                e - FLAG_NUM - 1
            } else {
                e - 1
            };

            let target = nums.get_mut(index as usize).unwrap();
            if *target < FLAG_NUM {
                *target += FLAG_NUM;
            }
        }

        let mut result = vec![];
        for (i, &e) in nums.iter().enumerate() {
            if e < FLAG_NUM {
                result.push(i as i32 + 1);
            }
        }
        result
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
}
