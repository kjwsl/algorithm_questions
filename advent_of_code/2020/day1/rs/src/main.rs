use std::{collections::HashSet, fs};

fn find_pair_for_sum(target_sum: u32, nums: &[u32]) -> Option<(u32, u32)> {
    let mut set = HashSet::new();
    for num in nums {
        if *num > target_sum {
            continue
        }

        if set.contains(&(target_sum - num)) {
            return Some((*num, target_sum - num));
        }
        set.insert(*num);
    }
    None
}

fn part1(input: &str) -> u32 {
    const TARGET_SUM: u32 = 2020;

    let nums = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    match find_pair_for_sum(TARGET_SUM, &nums) {
        Some(pair) => pair.0 * pair.1,
        None => panic!("No match found"),
    }
}

fn part2(input: &str) -> u32 {
    const TARGET_SUM: u32 = 2020;

    let nums = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for num in &nums {
        let pair = find_pair_for_sum(TARGET_SUM - *num, &nums);
        if let Some(pair) = pair {
            return num * pair.0 * pair.1;
        }
    }

    panic!("No match found");
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
