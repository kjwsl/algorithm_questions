use std::collections::{HashMap, HashSet};

fn get_common_char(strings: &[&str]) -> Option<char> {
    if strings.is_empty() {
        return None;
    }

    let mut iter = strings.iter();
    let first_set = match iter.next() {
        Some(s) => s.chars().collect::<HashSet<_>>(),
        None => panic!("What the heeeeeeeell"),
    };

    let common_set = iter.fold(first_set, |acc, s| {
        acc.intersection(&s.chars().collect::<HashSet<char>>())
            .cloned()
            .collect()
    });

    match common_set.into_iter().collect::<Vec<char>>()[..] {
        [common_char] => Some(common_char),
        [] => None,
        _ => None,
    }
}

fn get_priority(ch: char) -> i32 {
    if ch.is_ascii_lowercase() {
        ch as i32 - 'a' as i32 + 1
    } else if ch.is_ascii_uppercase() {
        ch as i32 - 'A' as i32 + 27
    } else {
        panic!("What the heeeeeeeeeeeeeell")
    }
}

pub fn solve(input: &str) -> i32 {
    let backpacks = input.lines().collect::<Vec<_>>();
    let groups = backpacks.iter().as_slice().chunks(3);
    let mut sum = 0;
    for group in groups {
        let common = get_common_char(group).expect("Nooooo wayyyyy");
        println!("group: {group:?}");
        println!("common: {common}");
        sum += get_priority(common);
    }

    sum
}
