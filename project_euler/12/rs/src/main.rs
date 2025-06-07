fn is_triangle(n: i32) -> bool {
    // Calculate k = (-1 + sqrt(1 + 8n)) / 2
    // If k is an integer, then n is a triangle number
    let discriminant = 1.0 + 8.0 * n as f64;
    let k = (-1.0 + discriminant.sqrt()) / 2.0;
    (k.fract() == 0.0) && (k > 0.0)
}

fn count_divisors(n: i32) -> usize {
    let mut count = 0;
    let sqrt_n = (n as f64).sqrt() as i32;
    
    for i in 1..=sqrt_n {
        if n % i == 0 {
            count += 1;
            // If i is not the square root, count its pair
            if i != n / i {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let res = (1..)
        .filter(|&n| is_triangle(n))
        .find(|&n| count_divisors(n) > 500)
        .unwrap();

    println!("{}", res);
}
