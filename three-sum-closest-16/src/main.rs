struct Solution {}
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        Self::merge_sort(&mut nums);
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                if sum < target {
                    j += 1;
                } else if sum > target {
                    k -= 1;
                } else {
                    return target;
                }
            }
        }
        closest
    }

    fn merge_sort(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }
        let mid = nums.len() / 2;
        Self::merge_sort(&mut nums[..mid]);
        Self::merge_sort(&mut nums[mid..]);
        let mut temp = Vec::with_capacity(nums.len());
        let mut i = 0;
        let mut j = mid;
        while i < mid && j < nums.len() {
            if nums[i] <= nums[j] {
                temp.push(nums[i]);
                i += 1;
            } else {
                temp.push(nums[j]);
                j += 1;
            }
        }
        temp.extend_from_slice(&nums[i..mid]);
        temp.extend_from_slice(&nums[j..]);
        nums.copy_from_slice(&temp);
    }
}
fn main() {
    let input = vec![-1, 2, 1, -4];
    let result = Solution::three_sum_closest(input, 1);
    println!("{:?}", result);
}
