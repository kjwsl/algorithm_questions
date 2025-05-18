struct Solution;
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|&word| word.starts_with(&pref)).count() as i32
    }
}
fn main() {
    println!("Hello, world!");
}
