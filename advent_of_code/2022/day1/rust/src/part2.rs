use std::{cmp::Reverse, collections::BinaryHeap};

pub fn solve(input: &str) -> i32 {
    let lines = input.lines();
    let mut top_list = Vec::new();

    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            top_list.push(current);
            current = 0;
            continue;
        }

        current += line
            .parse::<i32>()
            .expect("The line doesn't contain a number");
    }

    top_list.sort_by_key(|e| Reverse(*e));
    println!("{:?}", top_list);
    top_list[0..3].iter().sum()
}
