struct Solution {}
fn main() {}

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' | ']' | '}' => {
                    let top = stack.pop();
                    match (top, ch) {
                        (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => continue,
                        _ => return false,
                    }
                }
                _ => (),
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test1() {
    assert!(Solution::is_valid(String::from("()[]{}")));
}
