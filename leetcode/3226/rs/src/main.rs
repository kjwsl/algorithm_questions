struct Solution;

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n == k {
            return 0;
        } else if n < k {
            return -1;
        }

        let mut cnt = 0;

        for i in 0..32 {
            let bit = 1 << i;
            if (n & bit) != 0 && (k & bit) == 0 {
                cnt += 1;
            } else if (n & bit) == 0 && (k & bit) != 0 {
                return -1;
            }
        }
        // 1110 1101

        cnt
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let res = Solution::min_changes(13, 4);
        // 1101 vs 0100
        assert_eq!(res, 2);
    }

    #[test]
    fn test_2() {
        let res = Solution::min_changes(14, 13);
        // 1101 vs 0100
        assert_eq!(res, -1);
    }
}

fn main() {
    println!("Hello, world!");
}
