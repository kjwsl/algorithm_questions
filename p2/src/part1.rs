#[derive(Debug)]
pub enum Color {
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

pub fn solve(input: Vec<String>, threshold: (u32, u32, u32)) -> u32 {
    let mut sum = 0;
    for line in input {
        let line = line;
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 2 {
            println!("Invalid line: {}", line);
            continue;
        }

        let mut game_num: u32 = 0;
        if let Some(last_char) = parts[0].split(" ").last() {
            game_num = last_char.parse::<u32>().unwrap();
        }

        let iterations = parts[1].split(";");
        println!("Iterations: {:?}", iterations);

        let mut is_valid: bool = true;
        for it in iterations {
            let rgb: Vec<&str> = it.split(",").collect();
            println!("RGB: {:?}", rgb);
            for i in 0..rgb.len() {
                let bead = extract_beads(rgb[i]);
                if let Some(bead) = bead {
                    match bead {
                        Color::Red(num) => {
                            if num > threshold.0 {
                                is_valid = false;
                                break;
                            }
                        }
                        Color::Green(num) => {
                            if num > threshold.1 {
                                is_valid = false;
                                break;
                            }
                        }
                        Color::Blue(num) => {
                            if num > threshold.2 {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                } else {
                    println!("Invalid bead: {}", rgb[i]);
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            sum += game_num;
        }
        println!("Game number: {}", game_num);
        println!("Sum: {}", sum);
    }
    sum
}
