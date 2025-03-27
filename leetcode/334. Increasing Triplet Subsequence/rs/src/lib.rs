use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    // [1,5,0,4,1,3]
    // [7,8,5,4,1,9]
    // pairs of increasing numbers have to be within 0..n-2
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for num in nums {
            if first >= num {
                first = num;
            } else if second >= num {
                second = num;
            } else {
                return true;
            }
        }

        false
    }
}
