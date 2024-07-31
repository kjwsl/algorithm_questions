use std::io::Read;
use std::fs::File;
use std::env;

const NUM_STR: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn strnum_to_num(text: String) -> String {
    let new_text = text.clone();
    let mut pairs: Vec<(u8, usize)> = Vec::new();
    for (i, numstr) in Vec::from(NUM_STR).iter().enumerate() {
        pairs.push(((i + 1) as u8, new_text.find(numstr)));
    }
    println!("{:?}", pairs);
    new_text
}

fn do_the_thing(src: &str) -> u32 {
    let new_text = strnum_to_num(src.to_string());
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

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/sample.txt")?;
    let mut src = String::new(); 
    file.read_to_string(&mut src)?;
    let res = do_the_thing(&src);
    println!("{}", res);
    Ok(())
}
