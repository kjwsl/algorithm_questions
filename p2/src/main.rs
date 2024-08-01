use std::fs::File;
use std::io::{Read, Write};

const NUM_STR: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn evaluate_line(line: &str) -> u32 {
    if line.len() == 0 {
        return 0;
    }
    let mut nums: Vec<(usize, u32)> = vec![];
    for i in 0..NUM_STR.len() {
        line.match_indices(NUM_STR[i]).for_each(|(idx, _)| {
            nums.push((idx, i as u32 + 1));
        });
    }
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            nums.push((i, c.to_digit(10).unwrap() as u32));
        }
    }

    if nums.len() == 0 {
        return 0;
    }

    nums.sort_by(|a, b| a.0.cmp(&b.0));
    let res = nums[0].1 * 10 + nums.last().unwrap().1;
    print_text_in_box(
        vec![
            format!("")
        ]
    );
    res
}

fn do_the_thing(src: &str) -> u32 {
    let new_text = src;
    let lines: Vec<&str> = new_text.split('\n').collect();
    let mut digits: Vec<u32> = vec![];
    for line in lines {
        let res = evaluate_line(line);
        digits.push(res);
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
