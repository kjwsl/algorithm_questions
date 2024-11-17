use std::cmp::max;

struct Solution;
impl Solution {
    pub fn kadane(nums: Vec<i32>) -> i32 {
        let mut current_sum = nums[0];
        let mut max_sum = nums[0];

        for num in nums.iter().skip(1) {
            current_sum = max(*num, current_sum + *num);
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max_sum = Solution::kadane(nums.clone());

        let size = nums.len();
        let nums = nums
            .iter()
            .cycle()
            .skip(nums.len() - 1)
            .take(nums.len())
            .copied()
            .collect::<Vec<_>>();
        println!("{nums:?}");

        max_sum = max_sum.max(Solution::kadane(nums.clone()));

        max_sum
    }
}

fn main() {
    let ans = Solution::max_subarray_sum_circular(vec![5, -3, 5]);

    println!("Ans: {}", ans);
}
