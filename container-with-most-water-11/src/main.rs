struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let area = (right - left) as i32
                * if height[left] < height[right] {
                    left += 1;
                    height[left - 1]
                } else {
                    right -= 1;
                    height[right + 1]
                };
            max_area = if area > max_area { area } else { max_area };
        }
        max_area
    }
}
fn main() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = Solution::max_area(input);
    println!("{}", result);
}
