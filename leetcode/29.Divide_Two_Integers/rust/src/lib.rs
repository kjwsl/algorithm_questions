pub struct Solution;

// Not allowed to use the division, multiplication, or mod operator
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        } else if dividend == divisor {
            return 1;
        }

        let sign = if (dividend < 0) ^ (divisor < 0) { -1 } else { 1 };
        let dividend = dividend.abs() as i64;
        let divisor = divisor.abs() as i64;

        let mut i = 1;
        let mut acc = divisor;

        while acc <= dividend {
            acc <<= 1;
            i <<= 1;
        }

        i
    }
}

// 3 / 2 = 1
