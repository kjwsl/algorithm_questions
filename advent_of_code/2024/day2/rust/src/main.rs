mod part1;
mod part2;

fn main() {
    let path = "src/input.txt";
    let input =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read file: {}", path));

    let ans = part1::solve(&input);
    println!("Part 1: {}", ans);

    let ans = part2::solve(&input);
    println!("Part 2: {}", ans);
}
