struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let a = match nums.iter().enumerate().find(|&(_, &val)| val == target) {
            Some((idx, _)) => idx as i32,
            None => -1,
        };
        let b = match nums.iter().enumerate().rfind(|&(_, &val)| val == target) {
            Some((idx, _)) => idx as i32,
            None => -1,
        };

        vec![a, b]
    }
}
