fn is_palindrome(n: i64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let mut largest = 0;

    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if is_palindrome(product) && product > largest {
                largest = product;
            }
        }
    }

    println!("{}", largest);
}
