use std::io::{self, Read};

const NUM_STR: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


fn strnum_to_num(text: &String) -> String {
    let mut new_text = text.clone();
    let mut pairs: Vec<(u8, usize)> = Vec::new();
    for (i, numstr) in Vec::from(NUM_STR).iter().enumerate() {
        pairs.push(((i + 1) as u8, new_text.find(numstr).unwrap()));
    }
    println!("{}", new_text);
    new_text
}

fn do_the_thing(src: &str) -> u32 {
    let new_text = strnum_to_num(&src.to_string());
    let lines: Vec<&str> = new_text.split('\n').collect();
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
    let ra = 0..3;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let res = do_the_thing(input.as_str());
    println!("{}", res);
}
