struct Solution {}
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut start, mut end, mut sum) = (0, 0, 0);
        let mut min_len = std::i32::MAX as usize;

        while end < nums.len() {
            sum += nums[end];
            end += 1;

            while sum >= target {
                min_len = min_len.min(end - start);
                sum -= nums[start];
                start += 1;
            }
        }

        if min_len == std::i32::MAX as usize {
            0
        } else {
            min_len as i32
        }
    }
}
fn main() {
    let target = 7;
    let input = vec![2, 3, 1, 2, 4, 3];
    let result = Solution::min_sub_array_len(target, input.clone());
    println!("target: {}\ninput: {:?}\nresult: {}", target, input, result);
}
