use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let diag_idxs = Self::get_diagnal_idxs(nums.len() as i32);
        println!("{:?}", diag_idxs);
        nums.iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, &num)| {
                if diag_idxs.contains(&(i as i32)) {
                    Some(num)
                } else {
                    None
                }
            })
            .filter(|&num| Self::is_prime(num))
            .max()
            .unwrap_or(0)
        // *nums.iter().flatten().filter(|&num| Self::is_prime(*num)).max().unwrap_or(&0)
    }

    fn get_diagnal_idxs(n: i32) -> HashSet<i32> {
        let mut idxs = HashSet::new();

        for i in 0..n {
            idxs.insert((n + 1) * i);
            idxs.insert((n * n - 1) - (n - 1) * (i + 1));
        }

        idxs
    }

    fn is_prime(num: i32) -> bool {
        if num < 2 {
            return false;
        } else if num == 2 {
            return true;
        }

        // `num` must not be divisible by any number from 2 upto sqrt(num).
        (2..=num.isqrt()).all(|k| num % k != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime() {
        assert!(Solution::is_prime(2));
        assert!(Solution::is_prime(3));
        assert!(!Solution::is_prime(4));
        assert!(Solution::is_prime(5));
        assert!(!Solution::is_prime(6));
    }
}
fn main() {
    println!("Hello, world!");
}
