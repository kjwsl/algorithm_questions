pub struct Solution;

impl Solution {
    const PRIME_DIGITS: [char; 4] = ['2', '3', '5', '7'];
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let prime_idx_list: Vec<_> = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if Self::PRIME_DIGITS.contains(&c) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        println!("prime_idx_list: {:?}", prime_idx_list);

        Self::bp_rec(&s[..], 0, k, min_length, &prime_idx_list, 0)
    }

    fn bp_rec(
        s: &str,
        s_start: usize,
        k: i32,
        min_length: i32,
        prime_idx_list: &[usize],
        prime_start: usize,
    ) -> i32 {
        if prime_idx_list.len() == 0 {
            return 0;
        }

        if k == 0 {
            return 1;
        } else if k == 1 {
            if s.len() - s_start < min_length as usize {
                return 0;
            } else {
                return 1;
            }
        }
        if prime_idx_list.len() - prime_start < k as usize {
            return 0;
        }

        let mut res = 0;
        let first = prime_idx_list[prime_start];
        for idx in prime_start + 1..prime_idx_list.len() {
            let next = prime_idx_list[idx];

            if next - first < min_length as usize {
                continue;
            }

            if prime_idx_list[idx - 1] == next - 1 {
                continue;
            }

            res += Self::bp_rec(s, next, k - 1, min_length, prime_idx_list, idx);
        }
        res
    }
}
