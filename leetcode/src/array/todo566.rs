struct Solution;
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let num_rows = mat.len();
        let num_cols = mat.get(0).unwrap().len();
        if num_rows * num_cols != (r * c) as usize {
            return mat;
        }

        let mut vec = Vec::with_capacity(r as usize);
        for i in 0..r {
            let mut row = Vec::with_capacity(c as usize);
            for j in 0..c {
                let nth = i * c + j;
                row.push(
                    *mat.get(nth as usize / num_cols)
                        .unwrap()
                        .get(nth as usize % num_cols)
                        .unwrap(),
                );
            }
            vec.push(row);
        }
        vec
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );
}
