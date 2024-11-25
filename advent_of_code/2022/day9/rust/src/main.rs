mod part1;
mod part2;
fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Error reading input file");

    let and = part1::solve(&input);
    println!("Part 1: {}", and);

    // let and = part2::solve(&input);
    // println!("Part 2: {}", and);
}
