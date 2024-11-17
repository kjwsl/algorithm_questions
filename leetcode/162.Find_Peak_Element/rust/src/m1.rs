use core::slice::SlicePattern;

struct Solution;
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        match nums.iter().enumerate().max_by_key(|&(_, &val)| val) {
            Some((index, _)) => index as i32,
            None => 0,
        }
    }
}
