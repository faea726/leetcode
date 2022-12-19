struct Solution {}
impl Solution {
    fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i <= j {
            if nums[i].abs() > nums[j].abs() {
                result.push(nums[i] * nums[i]);
                i += 1;
            } else {
                result.push(nums[j] * nums[j]);
                j -= 1;
            }
        }
        result.reverse();
        result
    }
}

fn main() {
    let input = vec![-5, 4, 3, -9, 4, 8, 6];
    let result = Solution::sorted_squares(input);
    println!("{:?}", result)
}
