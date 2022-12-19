struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) -> *mut Vec<i32> {
        let total_len = nums.len();
        if total_len < 2 {
            return nums;
        };
        let mut l_pointer = 0;
        let mut r_pointer = 1;
        while r_pointer < total_len {
            if nums[l_pointer] != 0 {
                l_pointer += 1;
                r_pointer += 1;
            } else if nums[r_pointer] == 0 {
                r_pointer += 1;
            } else {
                let tmp = nums[r_pointer];
                nums[r_pointer] = nums[l_pointer];
                nums[l_pointer] = tmp;
            }
        }
        return nums;
    }
}
fn main() {
    let mut input = vec![0, 4, 0, 5, 6, 0, 9];
    Solution::move_zeroes(&mut input);
    println!("{:?}", input)
}
