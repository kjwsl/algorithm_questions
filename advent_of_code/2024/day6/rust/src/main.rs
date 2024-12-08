mod part1;
fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Can't read input.txt");

    let ans = part1::solve(&input);
    println!("Part 1: {}", ans);
}
