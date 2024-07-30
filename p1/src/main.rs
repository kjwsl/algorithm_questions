use std::io::{self, Read};
fn do_the_thing(text: &str) -> u32 {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut digits: Vec<u32> = vec![];
    for line in lines {
        let first = match line.chars().find_map(|c| c.to_digit(10)) {
            Some(digit) => digit,
            None => {
                continue;
            }
        };
        let second = match line.chars().rev().find_map(|c| c.to_digit(10)) {
            Some(digit) => digit,
            None => {
                continue;
            }
        };
        digits.push(first * 10 + second);
    }
    digits.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let res = do_the_thing(input.as_str());
    println!("{}", res);
}
