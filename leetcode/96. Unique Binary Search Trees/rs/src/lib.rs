pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        Self::num_trees_memo(n, &mut HashMap::new())
    }

    fn num_trees_memo(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&result) = cache.get(&n) {
            return result;
        }

        if n <= 1 {
            return 1;
        }

        let mut total = 0;
        for i in 1..=n {
            let left_count = Self::num_trees_memo(i - 1, cache);
            let right_count = Self::num_trees_memo(n - i, cache);
            total += left_count * right_count;
        }

        cache.insert(n, total);
        total
    }
}
