pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut nums = nums;
        nums.sort_unstable();
        nums.dedup();

        const INIT_CNT: i32 = 1;
        let mut max_cnt = INIT_CNT;
        let _ = nums.windows(2).fold(INIT_CNT, |cnt, nums| {
            let (a, b) = (nums[0], nums[1]);
            if a + 1 == b {
                let cnt = cnt + 1;
                max_cnt = max_cnt.max(cnt);
                cnt
            } else {
                INIT_CNT
            }
        });

        max_cnt
    }
}
