mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Failed to read the input file");

    let ans = part1::solve(&input);
    println!("Ans: {ans:?}");

    let ans = part2::solve(&input);
    println!("Ans: {ans:?}");
}
