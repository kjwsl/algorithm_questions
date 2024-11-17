pub struct Solution;

impl Solution {
    pub fn search_insert_rec(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
        let pivot = (start + end) / 2;

        if start == end {
            if nums[pivot] < target {
                return pivot as i32 + 1;
            }
            return pivot as i32;
        }

        let curr_num = nums[pivot];
        if pivot == 0 {
            if curr_num >= target {
                return 0;
            } else {
                return 1;
            }
        } else if pivot == nums.len() - 1 {
            if curr_num >= target {
                return pivot as i32;
            } else {
                return pivot as i32 + 1;
            }
        } else {
            let prev_num = nums[pivot - 1];
            if prev_num < target && curr_num >= target {
                return pivot as i32;
            }
        }

        if target > curr_num {
            Solution::search_insert_rec(nums, target, pivot + 1, end)
        } else {
            Solution::search_insert_rec(nums, target, start, pivot - 1)
        }
    }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if target < nums[0] {
            return 0;
        }
        if target > nums[nums.len() - 1] {
            return nums.len() as i32;
        }
        Solution::search_insert_rec(&nums, target, 0, nums.len() - 1)
    }
}

fn main() {
    let ans = Solution::search_insert(vec![1, 3, 5, 6], 5);
}

#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn test1() {
        let ans = Solution::search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(ans, 2)
    }

    #[test]
    fn test2() {
        let ans = Solution::search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(ans, 1)
    }

    #[test]
    fn test3() {
        let ans = Solution::search_insert(vec![1, 3, 5, 6], 7);
        assert_eq!(ans, 4)
    }
}
