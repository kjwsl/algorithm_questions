use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn part1(input: &str) -> usize {
        let nums: Vec<_> = input
            .split(" ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        Self::count_stones_after_blinks(&nums, 25)
    }

    pub fn part2(input: &str) -> usize {
        let nums: Vec<_> = input
            .split(" ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        Self::count_stones_after_blinks(&nums, 75)
    }

    // Count the number of stones after applying the blink operation
    fn count_stones_after_blinks(nums: &[i64], count: usize) -> usize {
        // Base case
        if count == 0 {
            return nums.len();
        }

        // Create a direct mapping from each initial number to its count after n blinks
        let mut memo: HashMap<(i64, usize), usize> = HashMap::new();
        let mut total = 0;

        for &num in nums {
            total += Self::count_after_blinks(num, count, &mut memo);
        }

        total
    }
    
    // Count how many stones result from a single number after n blinks
    fn count_after_blinks(num: i64, blinks: usize, memo: &mut HashMap<(i64, usize), usize>) -> usize {
        // Base case
        if blinks == 0 {
            return 1;
        }
        
        // Check memo
        let key = (num, blinks);
        if let Some(&count) = memo.get(&key) {
            return count;
        }
        
        // Compute result based on the blink rules
        let count = if num == 0 {
            // 0 becomes 1
            Self::count_after_blinks(1, blinks - 1, memo)
        } else if Self::count_digit(num) % 2 == 0 {
            // Even number of digits - split into two
            let (left, right) = Self::split_num_half(num).unwrap();
            Self::count_after_blinks(left, blinks - 1, memo) +
            Self::count_after_blinks(right, blinks - 1, memo)
        } else {
            // Odd number of digits - multiply by 2024
            Self::count_after_blinks(num * 2024, blinks - 1, memo)
        };
        
        // Memoize and return
        memo.insert(key, count);
        count
    }

    fn count_digit(num: i64) -> usize {
        if num < 10 {
            1
        } else {
            num.to_string().len()
        }
    }

    fn split_num_half(num: i64) -> Option<(i64, i64)> {
        if num < 10 {
            None
        } else {
            let num_str = num.to_string();
            let n = num_str.len() / 2;
            let (left, right) = num_str.split_at(n);
            Some((left.parse().unwrap(), right.parse().unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "125 17";

    #[test]
    fn test_blink_with_count() {
        let input = include_str!("input.txt");
        let nums: Vec<_> = input
            .split(" ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        let res = Solution::count_stones_after_blinks(&nums, 25);
        assert_eq!(res, 186424)
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(Solution::part1(SAMPLE1), 55312);
    }
}
