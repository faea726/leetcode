struct Solution {}
impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }

        (i + 1) as i32
    }
}
fn main() {
    let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{:?}", input);
    let counter = Solution::remove_duplicates(&mut input);
    println!("{:?}, nums = {:?}", counter, input)
}
