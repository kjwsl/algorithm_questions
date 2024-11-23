use std::{
    fs::{read_to_string, File},
    io::{BufReader, Read},
    path::Path,
};

mod part1;
mod part2;
fn main() {
    let input = read_to_string("src/input.txt").expect("Failed to read the file");

    let ans = part1::solve(&input);
    println!("Ans: {}", ans);

    let ans = part2::solve(&input);
    println!("Ans: {:?}", ans);
}
