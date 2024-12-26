mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading file");

    println!("Part 1: {}", part1::solve(&input));
}
