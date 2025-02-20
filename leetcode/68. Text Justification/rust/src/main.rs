use rust::Solution;

fn main() {
    let words = vec![
        "This".to_string(),
        "is".to_string(),
        "an".to_string(),
        "example".to_string(),
        "of".to_string(),
        "text".to_string(),
        "justification.".to_string(),
    ];
    let ans = Solution::full_justify(words, 16);
    println!("Answer: \n{}", ans.join("\n"));
}
