pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.iter().enumerate().find(|(_, &val)| val == target) {
            Some((index, _)) => index as i32,
            None => -1,
        }
    }
}
