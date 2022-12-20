struct Solution {}
impl Solution {
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

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;

        Self::merge_sort(&mut nums);

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let target = -nums[i];

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                if nums[l] + nums[r] == target {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                } else if nums[l] + nums[r] > target {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        res
    }
}

fn main() {
    let input = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(input);
    println!("{:?}", result);
}
