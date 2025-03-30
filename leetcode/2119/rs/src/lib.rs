pub struct Solution;
impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        Self::reverse(Self::reverse(num)) == num
    }

    fn reverse(num: i32) -> i32 {
        let mut num = num;
        let mut reversed = 0;
        while num > 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }
        reversed
    }
}
