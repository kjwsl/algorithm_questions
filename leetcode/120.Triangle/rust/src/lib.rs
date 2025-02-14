pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut memo = HashMap::new();

        Self::minimum_total_helper(&triangle, 0, 0, &mut memo)
    }

    fn minimum_total_helper(
        triangle: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if row == triangle.len() {
            return 0;
        }
        if let Some(&ans) = memo.get(&(row, col)) {
            return ans;
        }

        let left = Self::minimum_total_helper(triangle, row + 1, col, memo);
        let right = Self::minimum_total_helper(triangle, row + 1, col + 1, memo);
        let ans = triangle[row][col] + std::cmp::min(left, right);
        memo.insert((row, col), ans);
        ans
    }
}
