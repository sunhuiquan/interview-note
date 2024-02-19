struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        const PREFIX: i32 = 1000000;
        let mut nums = nums;
        let len = nums.len();
        for i in 0..len {
            let num = nums.get(i).unwrap();
            let idx = if *num > PREFIX { *num - PREFIX } else { *num } as usize;
            let value = nums.get_mut(idx).unwrap();
            if *value > PREFIX {
                return idx as i32;
            } else {
                *value += PREFIX;
            }
        }
        -1
    }
}

#[test]
fn test1() {}
