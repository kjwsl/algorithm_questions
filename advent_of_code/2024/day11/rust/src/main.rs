mod solution;
use crate::solution::Solution;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading file");

    let mut sol = Solution::new(&input);

    println!("part1: {}", sol.solve_part1());
    println!("part2: {}", sol.solve_part2());
}
