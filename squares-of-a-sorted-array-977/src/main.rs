struct Solution {}
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        if a.len() == 1 {
            return vec![a[0] * a[0]];
        }
        let mut res = vec![0; a.len()];
        let mut i = 0;
        let mut j = a.len() - 1;
        let mut k = a.len() - 1;
        while i <= j {
            if a[i].pow(2) >= a[j].pow(2) {
                res[k] = a[i] * a[i];
                i += 1;
            } else {
                res[k] = a[j] * a[j];
                j -= 1;
            }
            k -= 1;
        }
        res
    }
}

fn main() {
    let input = vec![-7, -3, 2, 3, 11];
    let result = Solution::sorted_squares(input);
    println!("{:?}", result);
}
