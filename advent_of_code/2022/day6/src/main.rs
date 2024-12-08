mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Failed to read input.txt");

    let ans = part1::solve(&input);
    println!("Part 1: {}", ans);

    let ans = part2::solve(&input);
    println!("Part 2: {}", ans);
}
