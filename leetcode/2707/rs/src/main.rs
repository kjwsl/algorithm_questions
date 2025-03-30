use std::fmt::{Debug, Display};

pub struct Solution;

fn eq<T: PartialEq + Debug>(a: &T, b: &T) {
    let is_eq = a == b;
    println!("{:?} {} {:?}", a, if is_eq { "==" } else { "!=" }, b);
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut exclude = vec![false; s.len()];

        for ref word in dictionary {
            let mut start = 0;
            while let Some(pos) = s[start..].find(word) {
                for i in 0..word.len() {
                    exclude[pos + i] = true;
                }
                start += pos + 1;
            }
        }


        exclude.iter().filter(|&&x| !x).count() as i32
    }

}

fn main() {
    let s = "leetcoder".to_string();
    let dictionary = vec!["leet".to_string(), "code".to_string()];
    let result = Solution::min_extra_char(s, dictionary);
    eq(&result, &1);
}
