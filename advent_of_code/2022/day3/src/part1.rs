use std::collections::HashMap;

fn get_common_char(left: &str, right: &str) -> Option<char> {
    let mut map = HashMap::new();
    for c in left.chars() {
        map.insert(c, true);
    }

    for c in right.chars() {
        if let Some(&val) = map.get(&c) {
            if val {
                return Some(c);
            }
        }
    }
    None
}

pub fn solve(input: &str) -> i32 {
    let backpacks = input.lines();
    let mut sum = 0;
    for backpack in backpacks {
        let (left, right) = backpack.split_at(backpack.len() / 2);
        let common_char = get_common_char(left, right).expect("No way");

        let val = if common_char.is_ascii_lowercase() {
            common_char as i32 - 'a' as i32 + 1
        } else if common_char.is_ascii_uppercase() {
            common_char as i32 - 'A' as i32 + 27
        } else {
            panic!("What the heeeeeeeeeeeeeell")
        };

        sum += val;
    }
    sum
}
