mod part1;
mod part2;
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    println!("Part 1: {}", part1::solve(&input));
    println!("Part 2: {}", part2::solve(&input));
}
