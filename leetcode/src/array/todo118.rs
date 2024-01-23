struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        for i in 1..num_rows as usize {
            let mut vec = vec![1];
            for j in 1..=i as usize {
                let x1 = res.get(i - 1).unwrap().get(j - 1).unwrap();
                if let Some(x2) = res.get(i - 1).unwrap().get(j) {
                    vec.push(*x1 + *x2);
                } else {
                    vec.push(*x1);
                }
            }
            res.push(vec);
        }
        res
    }
}

// 输入: numRows = 5
// 输出: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]

#[test]
fn test1() {
    let v1 = Solution::generate(1);
    let v2 = Solution::generate(2);
    let v3 = Solution::generate(5);

    assert_eq!(v1, vec![vec![1]]);
    assert_eq!(v2, vec![vec![1], vec![1, 1]]);
    assert_eq!(
        v3,
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
}
