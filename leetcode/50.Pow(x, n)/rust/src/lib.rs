pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        } else if n < 0 {
            return 1.0 / x * Solution::my_pow(1.0 / x, -(n + 1));
        }

        if n % 2 == 0 {
            return Solution::my_pow(x * x, n / 2);
        } else {
            return x * Solution::my_pow(x * x, n / 2);
        }
    }
}
