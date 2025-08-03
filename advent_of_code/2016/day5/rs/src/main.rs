use md5::Context;

fn part1(input: &str) -> String {
    let door_id = input.trim();

    let mut password = String::new();
    let mut index = 0;

    while password.len() < 8 {
        let candidate = format!("{}{}", door_id, index);
        let mut hasher = Context::new();
        hasher.consume(candidate.as_bytes());
        let hash = hasher.finalize();

        let bytes = hash
            .into_iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        if bytes.starts_with("00000") {
            password.push(bytes.chars().nth(5).unwrap());
        }

        index += 1;
    }

    password
}

fn part2(input: &str) -> String {
    let door_id = input.trim();

    let mut password = "        ".to_string();
    let mut index = 0;

    while password.len() < 8 || password.chars().any(|c| !c.is_ascii_hexdigit()) {
        let candidate = format!("{}{}", door_id, index);
        let mut hasher = Context::new();
        hasher.consume(candidate.as_bytes());
        let hash = hasher.finalize();

        let bytes = hash
            .into_iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        if bytes.starts_with("00000") {
            let pos_str = bytes.chars().nth(5).unwrap();
            if ('0'..='7').contains(&pos_str) {
                let pos = pos_str.to_digit(10).unwrap() as usize;
                let value = bytes.chars().nth(6).unwrap();

                if password.chars().nth(pos).unwrap() == ' ' {
                    password.replace_range(pos..pos + 1, &value.to_string());
                }
            }
        }

        index += 1;
    }

    password
}

fn main() {
    let door_id = std::fs::read_to_string("../input.txt").unwrap();

    println!("Part 1: {}", part1(&door_id));
    println!("Part 2: {}", part2(&door_id));
}
