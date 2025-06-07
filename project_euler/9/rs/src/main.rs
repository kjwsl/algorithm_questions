fn main() {
    let target_sum = 1000;
    let mut largest_product = 0;
    
    'outer: for a in 1..target_sum {
        for b in (a + 1)..(target_sum - a) {
            let c = target_sum - a - b;
            
            // Ensure c is positive and greater than b to avoid duplicates
            if c > b && a * a + b * b == c * c {
                largest_product = a * b * c;
                break 'outer; // Found the unique solution
            }
        }
    }
    
    println!("{}", largest_product);
}
