struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for chunk in nums.windows(3) {
            if let Some(mid) = chunk[1].checked_div_euclid(2) {
                if (chunk[0] + chunk[2]) == mid && mid * 2 == chunk[1] {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
fn main() {
    println!("Hello, world!");
}
