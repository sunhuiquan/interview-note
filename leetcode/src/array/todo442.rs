struct Solution {}
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        const PREFIX: i32 = 1000000;
        let mut nums = nums;
        let mut res = Vec::new();

        let len = nums.len();
        for i in 0..len {
            let num = nums.get(i).unwrap();
            let idx = if *num > PREFIX { *num % PREFIX } else { *num } as usize - 1;

            let mut_val = nums.get_mut(idx).unwrap();
            if *mut_val > PREFIX {
                res.push(idx as i32 + 1);
            } else {
                *mut_val += PREFIX;
            }
        }
        res
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![2, 3]
    );
}
