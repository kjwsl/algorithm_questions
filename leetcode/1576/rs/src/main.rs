fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut chars: Vec<_> = s.chars().collect();

        for i in 0..chars.len() {
            if chars[i] == '?' {
                let mut new_ch = b'a';
                if let Some(prev_idx) = i.checked_sub(1) {
                    if let Some(prev) = chars.get(prev_idx) {
                        if *prev == new_ch as char {
                            new_ch += 1;
                        }
                    }
                }
                if let Some(next) = chars.get(i + 1) {
                    if *next == new_ch as char {
                        new_ch += 1;
                    }
                }
                if let Some(prev_idx) = i.checked_sub(1) {
                    if let Some(prev) = chars.get(prev_idx) {
                        if *prev == new_ch as char {
                            new_ch += 1;
                        }
                    }
                }

                chars[i] = new_ch as char;
            }
        }

        chars.iter().collect()
    }
}
