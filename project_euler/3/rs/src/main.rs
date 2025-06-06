fn main() {
    const TARGET: i64 = 600851475143;
    let mut number = TARGET;
    let mut largest = 2;

    for i in 2..=((TARGET as f64).sqrt() as i64) {
        while number % i == 0 {
            if i > largest {
                largest = i;
            }
            number /= i;
        }
    }


    println!("{}", largest);
}
