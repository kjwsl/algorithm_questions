pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut even_len = 0;
        let mut odd_len = 0;
        let mut i = 0;
        let mut j = 1;

        while j < nums.len() {
            if (nums[i] + nums[j]) % 2 == 0 {
                even_len += 1;
                i = j;
            }
            j += 1;
        }

        i = 0;
        j = 1;

        while j < nums.len() {
            if (nums[i] + nums[j]) % 2 != 0 {
                odd_len += 1;
                i = j;
            }
            j += 1;
        }

        even_len.max(odd_len) + 1
    }
}

fn main() {
    let sample1 = vec![1, 2, 3, 4];
    assert_eq!(Solution::maximum_length(sample1), 4);

    let sample2 = vec![1, 2, 1, 1, 2, 1, 2];
    assert_eq!(Solution::maximum_length(sample2), 6);

    let sample3 = vec![1, 3];
    assert_eq!(Solution::maximum_length(sample3), 2);

    let sample4 = vec![1, 5, 9, 4, 2];
    assert_eq!(Solution::maximum_length(sample4), 3);
}
