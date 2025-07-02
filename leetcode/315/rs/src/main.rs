use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        let mut sorted: Vec<i32> = vec![];

        for &num in nums.iter().rev() {
            let index = match sorted.binary_search(&num) {
                Ok(i) | Err(i) => i,
            };
            res.push(index as i32);
            sorted.insert(index, num);
        }

        res.reverse();
        res
    }
}
