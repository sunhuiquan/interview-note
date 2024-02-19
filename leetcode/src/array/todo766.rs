struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let num_row = matrix.len();
        let num_col = matrix.first().unwrap().len();
        for i in 0..num_row - 1 {
            let row = matrix.get(i).unwrap();
            let next_row = matrix.get(i + 1).unwrap();
            if row[..num_col - 1] != next_row[1..] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test1() {
    assert!(Solution::is_toeplitz_matrix(vec![
        vec![1, 2, 3, 4],
        vec![5, 1, 2, 3],
        vec![9, 5, 1, 2]
    ]));

    assert!(!Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]));
}
