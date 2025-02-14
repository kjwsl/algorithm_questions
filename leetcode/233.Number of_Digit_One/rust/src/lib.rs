pub struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0;

        let mut i = 1;
        while i <= n {
            let divider = i * 10;
            count += (n / divider) * i + std::cmp::min(std::cmp::max(n % divider - i + 1, 0), i);
            i *= 10;
        }

        count
    }
}
