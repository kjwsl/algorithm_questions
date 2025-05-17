pub struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let found = words.iter().find(|&word| Self::is_palindrome(word));
        match found {
            Some(str) => str.to_owned(),
            None => "".to_string(),
        }
    }

    fn is_palindrome(string: &str) -> bool {
        let mid = string.len().div_euclid(2);
        for (a, b) in string.as_bytes()[0..mid]
            .iter()
            .zip(string.as_bytes()[mid..].iter().rev())
        {
            if *a != *b {
                return false;
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}
