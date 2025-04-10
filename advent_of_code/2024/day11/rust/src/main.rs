use rust::Solution;

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", Solution::part1(input));
    println!("part2: {}", Solution::part2(input));
}
