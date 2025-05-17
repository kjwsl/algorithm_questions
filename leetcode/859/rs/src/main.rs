struct Solution;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        if s == goal {
            let mut ch_cnt = [0; 26];

            for &byte in s.as_bytes() {
                let idx = (byte - b'a') as usize;
                if ch_cnt[idx] == 1 {
                    return true;
                }
                ch_cnt[idx] += 1;
            }
        }
        let mut diffs = vec![];

        for (i, (ch1, ch2)) in s.chars().zip(goal.chars()).enumerate() {
            if ch1 != ch2 {
                diffs.push(i);
                if diffs.len() > 2 {
                    return false;
                }
            }
        }

        if diffs.len() == 2 {
            let a1 = s.as_bytes()[diffs[0]];
            let b1 = s.as_bytes()[diffs[1]];

            let a2 = goal.as_bytes()[diffs[0]];
            let b2 = goal.as_bytes()[diffs[1]];

            if a1 == b2 && b1 == a2 {
                return true;
            }
        }

        false
    }
}
fn main() {
    println!("Hello, world!");
}
