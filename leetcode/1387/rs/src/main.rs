pub struct Solution;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        if lo == hi {
            return lo;
        }

        let mut power_list: Vec<_> = (lo..=hi).map(|n| (n, Self::get_power(n,0))).collect();
        power_list.sort_by(|a, b| a.1.cmp(&b.1));

        power_list[k as usize - 1].0
    }

    fn get_power(n: i32, cnt: i32) -> i32 {
        if n == 1 {
            return cnt;
        }

        if n % 2 == 0 {
            Self::get_power(n / 2, cnt + 1)
        } else {
            Self::get_power(3 * n + 1, cnt + 1)
        }
    }
}

fn main() {
    assert_eq!(Solution::get_kth(12, 15, 2), 13);
    assert_eq!(Solution::get_kth(7, 11, 4), 7);
}
