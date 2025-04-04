use rs::Solution;


fn main() {
    let input = include_str!("input.txt");
    println!("input: {}", input);
    let res = Solution::part1(input);
    println!("Res: {}", res);
}
