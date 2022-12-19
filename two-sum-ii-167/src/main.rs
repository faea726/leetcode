struct Solution {}
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // Define pointers
        let mut l_index = 0;
        let n = numbers.len();
        let mut r_index = n - 1;

        // 2 pointers find the sum
        while l_index < r_index {
            let sum = &numbers[l_index] + &numbers[r_index];
            if sum == target {
                return vec![(l_index + 1) as i32, (r_index + 1) as i32];
            } else if sum < target {
                l_index += 1;
            } else if sum > target {
                r_index -= 1;
            }
        }
        return vec![-1, -1];
    }
}

fn main() {
    let r_1 = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", r_1);

    let r_2 = Solution::two_sum(vec![2, 3, 4], 6);
    println!("{:?}", r_2);

    let r_3 = Solution::two_sum(vec![-1, 0], -1);
    println!("{:?}", r_3);

    let r_4 = Solution::two_sum(vec![1, 2, 4, 6, 10, 3, 10], -1);
    println!("{:?}", r_4);
}
