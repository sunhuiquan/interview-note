struct Solution;

fn main() {
    let mut input = vec![1];
    Solution::move_zeroes(&mut input);
    println!("{:?}", input);
}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut li = 0;
        let sz = nums.len();
        for ri in 0..sz {
            if li >= sz {
                break;
            }

            while li < sz && nums[li] != 0 {
                li += 1;
            }

            if nums[ri] != 0 && ri > li {
                nums[li] = nums[ri];
                nums[ri] = 0;
                li += 1;
            }
        }
    }
}
