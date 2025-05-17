struct Solution;

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut n = n;
        let mut a = n * 2;
        let mut b = n * 3;
        let mut digits = [0; 10];

        if a >= 1000 || b >= 1000 {
            return false;
        }

        for _ in 0..3 {
            let digit = (n % 10) as usize;
            digits[digit] += 1;
            n /= 10;

            let digit = (a % 10) as usize;
            digits[digit] += 1;
            a /= 10;

            let digit = (b % 10) as usize;
            digits[digit] += 1;
            b /= 10;
        }

        if digits[0] != 0 {
            return false;
        }

        for digit in digits.iter().skip(1) {
            if *digit != 1 {
                return false;
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}
