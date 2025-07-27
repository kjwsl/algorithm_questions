use std::collections::HashMap;

const INPUT_PATH: &str = "../input.txt";

fn part1(input: &str) -> String {
    let mut counters = vec![];

    let lines = input.lines();

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if counters.len() <= i {
                counters.push(HashMap::new());
            }

            let count = counters[i].entry(c).or_insert(0);
            *count += 1;
        }
    }

    counters
        .iter()
        .map(|map| map.iter().max_by_key(|(_, v)| *v).unwrap().0)
        .collect()
}

fn part2(input: &str) -> String {
    let mut counters = vec![];

    let lines = input.lines();

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if counters.len() <= i {
                counters.push(HashMap::new());
            }

            let count = counters[i].entry(c).or_insert(0);
            *count += 1;
        }
    }

    counters
        .iter()
        .map(|map| map.iter().min_by_key(|(_, v)| *v).unwrap().0)
        .collect()
}

fn main() {
    let input = std::fs::read_to_string(INPUT_PATH)
        .unwrap_or_else(|_| panic!("Failed to read {}", INPUT_PATH));

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
