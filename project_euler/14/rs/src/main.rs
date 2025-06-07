fn collatz(n: i128) -> i128 {
    match n {
        n if n % 2 == 0 => n / 2,
        _ => 3 * n + 1,
    }
}

fn main() {
    let res = (1..1_000_000)
        .map(|num| {
            let mut n = num;
            let mut cnt = 1;
            while n != 1 {
                n = collatz(n);
                cnt += 1;
            }
            (num, cnt)
        })
        .max_by_key(|(_, cnt)| *cnt)
        .unwrap()
        .0;

    println!("{}", res);
}
