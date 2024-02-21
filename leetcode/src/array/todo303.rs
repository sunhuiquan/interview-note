fn main() {}

#[allow(dead_code)]
struct NumArray {
    sum_nums: Vec<i32>,
}

#[allow(dead_code)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum_nums = Vec::with_capacity(nums.len());
        let mut sum = 0;
        for num in nums {
            sum += num;
            sum_nums.push(sum);
        }
        NumArray { sum_nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left >= 1 {
            *self.sum_nums.get(right as usize).unwrap()
                - *self.sum_nums.get(left as usize - 1).unwrap()
        } else {
            *self.sum_nums.get(right as usize).unwrap()
        }
    }
}

#[test]
fn test1() {
    let obj = NumArray::new(vec![-4, -5]);
    assert_eq!(obj.sum_range(1, 1), -5);
}
