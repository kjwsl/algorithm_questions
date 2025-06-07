fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let res = (2..2_000_000)
        .filter(|&n| is_prime(n))
        .sum::<u64>();

    println!("{}", res);
}
