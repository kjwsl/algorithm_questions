use std::iter;

fn is_prime(n: u64) -> bool {
    match n {
        0..=1 => false,
        2 => true,
        n if n % 2 == 0 => false,
        _ => {
            let limit = (n as f64).sqrt() as u64;
            (3..=limit).step_by(2).all(|i| n % i != 0)
        }
    }
}

fn main() {
    let res = iter::once(2)
        .chain((3..).step_by(2).filter(|&n| is_prime(n)))
        .nth(10000)
        .unwrap();

    println!("{}", res);
}
