pub struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut max_two = [(i32::MIN, i32::MIN); 10];

        for &num in &nums {
            let max_digit = Self::max_digit(num) as usize;
            let (largest, second_largest) = max_two[max_digit];

            if num >= largest {
                max_two[max_digit] = (num, largest);
            } else if num > second_largest {
                max_two[max_digit] = (largest, num);
            }
        }

        max_two
            .iter()
            .map(|(largest, second_largest)| {
                if *second_largest < 0 {
                    -1
                } else {
                    largest + second_largest
                }
            })
            .max()
            .unwrap_or(-1)
    }

    fn max_digit(num: i32) -> i32 {
        let mut n = num.abs();
        let mut max_digit = 0;
        while n > 0 {
            max_digit = max_digit.max(n % 10);
            n /= 10;
        }
        max_digit
    }
}
