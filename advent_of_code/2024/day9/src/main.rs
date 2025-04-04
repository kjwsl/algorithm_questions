use rs::Solution;


fn main() {
    let input = include_str!("input.txt");
    println!("input: {}", input);

    let res = Solution::part1(input);
    println!("Part 1: {}", res);

    let res = Solution::part2(input);
    println!("Part 2: {}", res);

}
