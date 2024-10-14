use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn extract_beads(text: &str) -> Option<Color> {
    let mut num = 0;
    let mut color_str = String::new();
    text.chars().into_iter().for_each(|c| {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap();
        } else {
            color_str.push(c);
        }
    });
    match color_str.trim().to_lowercase().as_str() {
        "red" => return Some(Color::Red(num)),
        "green" => return Some(Color::Green(num)),
        "blue" => return Some(Color::Blue(num)),
        _ => return None,
    }
}

fn solve(input: Vec<String>, threshold: (u32, u32, u32), part: u8) -> u32 {
    let mut sum = 0;
    for line in input {
        let line = line;
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 2 {
            continue;
        }

        let mut game_num: u32 = 0;
        if let Some(last_char) = parts[0].split(" ").last() {
            game_num = last_char.parse::<u32>().unwrap();
        }

        let iterations = parts[1].split(";").collect::<Vec<&str>>();

        let mut min_rgb = (0, 0, 0);
        let mut is_valid: bool = true;
        for it in iterations {
            let rgb: Vec<&str> = it.split(",").collect();
            for i in 0..rgb.len() {
                let bead = extract_beads(rgb[i]);
                if let Some(bead) = bead {
                    match bead {
                        Color::Red(num) => {
                            if part == 1 && num > threshold.0 {
                                is_valid = false;
                                break;
                            }
                            min_rgb.0 = std::cmp::max(min_rgb.0, num);
                        }
                        Color::Green(num) => {
                            if part == 1 && num > threshold.1 {
                                is_valid = false;
                                break;
                            }
                            min_rgb.1 = std::cmp::max(min_rgb.1, num);
                        }
                        Color::Blue(num) => {
                            if part == 1 && num > threshold.2 {
                                is_valid = false;
                                break;
                            }
                            min_rgb.2 = std::cmp::max(min_rgb.2, num);
                        }
                    }
                } else {
                    is_valid = false;
                    break;
                }
            }
        }
        if part == 1 && is_valid {
            sum += game_num;
        } else if part == 2 {
            sum += min_rgb.0 * min_rgb.1 * min_rgb.2;
        }
    }
    return sum;
}

fn main() -> io::Result<()> {
    let path = Path::new("sample1.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    // Red, Green, Blue
    let threshold = (12, 13, 14);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let ans = solve(lines, threshold, 1);
    println!("Answer for Part 1!!: {}", ans);

    let path = Path::new("sample2.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let ans = solve(lines, threshold, 2);
    println!("Answer for Part 2!!: {}", ans);

    Ok(())
}
