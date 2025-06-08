use rs::Solution;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read input file");
    println!("part1: {}", Solution::part1(&input));
    println!("part2: {}", Solution::part2(&input));

}
