struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        height.len() as i32
    }
}
fn main() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = Solution::max_area(input);
    println!("{}", result);
}
