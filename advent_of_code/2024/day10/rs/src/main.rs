use rs::Solution;

fn main() {
    let input = include_str!("input.txt");
    let part1 = Solution::part1(input);
    println!("Part 1: {}", part1);
    let part2 = Solution::part2(input);
    println!("Part 2: {}", part2);
}
