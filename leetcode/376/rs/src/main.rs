pub struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        let mut cnt = 1;
        let mut start = 1;
        while nums[start] == nums[start - 1] {
            start += 1;
            if start >= nums.len() {
                return 1;
            }
        }
        let mut expect_positive = nums[start] > nums[start - 1];
        for i in start..nums.len() {
            let curr = nums[i];
            let prev = nums[i - 1];
            // When it's wiggly
            if (expect_positive && curr > prev) || (!expect_positive && curr < prev) {
                cnt += 1;
                expect_positive = !expect_positive;
            }
        }

        cnt
    }
}

fn main() {
    let sample1 = vec![1, 7, 4, 9, 2, 5];
    assert_eq!(Solution::wiggle_max_length(sample1), 6);

    let sample2 = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    assert_eq!(Solution::wiggle_max_length(sample2), 7);

    let sample3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(Solution::wiggle_max_length(sample3), 2);
}
