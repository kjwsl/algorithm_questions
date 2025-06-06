fn main() {
    println!("{}", (1..=100).sum::<u64>().pow(2) - (1..=100).map(|n| n * n).sum::<u64>());
}
