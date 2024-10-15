use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::slice::Windows;

struct NumString {
    start: usize,
    end: usize,
    number: String,
}

impl NumString {
    fn new(start: usize, end: usize, number: String) -> NumString {
        NumString { start, end, number }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_next_num(text: &str, pos: usize) -> NumString {
    let mut start = pos;
    let mut end = pos;
    let mut found = false;
    let mut number = String::new();
    text.chars().into_iter().for_each(|c| {
        if c.is_digit(10) {
            if !found {
                start = pos;
                found = true;
                number.push(c);
            }
        } else {
            if found {
                end = pos - 1;
                return;
            }
        }
    });
    NumString::new(start, end, number)
}

fn is_part_number(lines: &Vec<String>, num: &NumString, line_num: usize) -> bool {
    let first = (( line_num - 1 ).max(0), )
    
}

fn solve_p1(lines: &Vec<String>) -> u32 {
    let mut sum = 0u32;
    for i in 0..lines.len() {
        let line = &lines[i];
        let mut pos = 0;
        while pos < line.len() {
            let num = find_next_num(line, pos);
            if is_part_number(lines, &num, i) {
                sum += num.number.parse::<u32>().unwrap();
            }
            pos = num.end + 1;
        }
    }
    sum
}

fn main() {
    let mut ans1: u32 = 0;
    if let Ok(lines) = read_lines("p1.txt") {
        if let Ok(vecLines) = lines.collect::<Result<Vec<String>, _>>() {
            ans1 = solve_p1(&vecLines);
        }
    }

    println!("Part 1: {}", ans1);
}
