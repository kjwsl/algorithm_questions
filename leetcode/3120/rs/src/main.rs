struct Solution;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        const ALPHABET_COUNT: usize = (b'Z' - b'A' + 1) as usize;
        let mut cnt = 0;
        let mut ch_map = [0; ALPHABET_COUNT * 2];
        for &ch in word.as_bytes() {
            let idx = if ch.is_ascii_lowercase() {
                (ch - b'a') as usize
            } else if ch.is_ascii_uppercase() {
                (ch - b'A') as usize + ALPHABET_COUNT
            } else {
                panic!("WTF????");
            };
            ch_map[idx] += 1;
        }

        for i in 0..ALPHABET_COUNT {
            if ch_map[i] > 0 && ch_map[i + ALPHABET_COUNT] > 0 {
                cnt += 1;
            }
        }

        cnt
    }
}
fn main() {
    println!("Hello, world!");
}
