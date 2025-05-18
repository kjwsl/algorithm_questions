struct Solution;
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        nums.iter()
            .enumerate()
            .filter_map(|(i, &num)| if num == target { Some(i as i32) } else { None })
            .collect::<Vec<i32>>()
    }
}
fn main() {
    println!("Hello, world!");
}
