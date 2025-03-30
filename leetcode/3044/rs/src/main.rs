use std::collections::{HashMap, HashSet};
use std::cmp::max;
struct Solution;

impl Solution {
    pub fn most_frequent_prime(matrix: Vec<Vec<i32>>) -> i32 {
        // Handle edge case: empty matrix
        if matrix.is_empty() || matrix[0].is_empty() {
            return -1;
        }
        
        let rows = matrix.len();
        let cols = matrix[0].len();
        
        // Pre-allocate HashMap with reasonable capacity
        // Estimate: at most rows*cols*8 different primes (one from each cell in each direction)
        let estimated_capacity = rows * cols * 8;
        let mut prime_counts: HashMap<i32, i32> = HashMap::with_capacity(estimated_capacity);
        
        // Cache for prime numbers to avoid repeated calculations
        let mut prime_cache: HashSet<i32> = HashSet::with_capacity(estimated_capacity);
        let mut not_prime_cache: HashSet<i32> = HashSet::new();
        
        // Define the 8 directions: E, SE, S, SW, W, NW, N, NE
        let directions = [
            (0, 1), (1, 1), (1, 0), (1, -1), 
            (0, -1), (-1, -1), (-1, 0), (-1, 1)
        ];
        
        // Traverse the matrix
        for r in 0..rows {
            for c in 0..cols {
                // Explore all 8 directions from each cell
                for (dr, dc) in directions.iter() {
                    let mut current_row = r as i32;
                    let mut current_col = c as i32;
                    let mut number = 0;
                    
                    // Generate numbers along the path
                    while current_row >= 0 && current_row < rows as i32 &&
                          current_col >= 0 && current_col < cols as i32 {
                        
                        number = number * 10 + matrix[current_row as usize][current_col as usize];
                        
                        // Only check primality for numbers > 10
                        if number > 10 {
                            // Use cache to avoid redundant primality checks
                            let is_prime = if prime_cache.contains(&number) {
                                true
                            } else if not_prime_cache.contains(&number) {
                                false
                            } else {
                                let result = Self::is_prime(number);
                                if result {
                                    prime_cache.insert(number);
                                } else {
                                    not_prime_cache.insert(number);
                                }
                                result
                            };
                            
                            if is_prime {
                                *prime_counts.entry(number).or_insert(0) += 1;
                            }
                        }
                        
                        // Move in the current direction
                        current_row += dr;
                        current_col += dc;
                    }
                }
            }
        }
        
        // Find the most frequent prime, using largest in case of ties
        if prime_counts.is_empty() {
            return -1;
        }
        
        let mut most_frequent_prime = -1;
        let mut max_frequency = 0;
        
        // Optimization: use fold to avoid multiple conditional checks
        most_frequent_prime = prime_counts.iter().fold(-1, |current_max, (&prime, &frequency)| {
            if frequency > max_frequency || (frequency == max_frequency && prime > current_max) {
                max_frequency = max(max_frequency, frequency);
                prime
            } else {
                current_max
            }
        });
        
        most_frequent_prime
    }
    
    // Efficient primality check
    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        
        true
    }
}

fn main() {
    // Example usage
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    println!("Most frequent prime: {}", Solution::most_frequent_prime(matrix));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_matrix() {
        let matrix: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::most_frequent_prime(matrix), -1);
        
        let matrix: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(Solution::most_frequent_prime(matrix), -1);
    }
    
    #[test]
    fn test_single_cell() {
        let matrix = vec![vec![7]];
        assert_eq!(Solution::most_frequent_prime(matrix), -1); // 7 is <= 10
        
        let matrix = vec![vec![11]];
        assert_eq!(Solution::most_frequent_prime(matrix), 11); // 11 is prime and > 10
    }
    
    #[test]
    fn test_no_valid_primes() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4]
        ];
        assert_eq!(Solution::most_frequent_prime(matrix), -1); // No primes > 10
    }
    
    #[test]
    fn test_multiple_primes_same_frequency() {
        let matrix = vec![
            vec![1, 1, 3],
            vec![1, 3, 1],
            vec![3, 1, 1]
        ];
        // This matrix forms 13 and 31 with same frequency, 31 is larger
        assert_eq!(Solution::most_frequent_prime(matrix), 31);
    }
    
    #[test]
    fn test_non_square_matrix() {
        let matrix = vec![
            vec![1, 9, 7],
            vec![3, 1, 1]
        ];
        assert_eq!(Solution::most_frequent_prime(matrix), 197); // 197 is prime
    }
    
    #[test]
    fn test_large_primes() {
        let matrix = vec![
            vec![7, 3, 1, 9],
            vec![2, 1, 1, 1],
            vec![1, 1, 4, 1],
            vec![3, 7, 9, 7]
        ];
        // This should find some large primes formed by concatenation
        let result = Solution::most_frequent_prime(matrix);
        assert!(result > 10 && Solution::is_prime(result));
    }
    
    #[test]
    fn test_is_prime() {
        assert!(!Solution::is_prime(1));
        assert!(Solution::is_prime(2));
        assert!(Solution::is_prime(3));
        assert!(!Solution::is_prime(4));
        assert!(Solution::is_prime(5));
        assert!(!Solution::is_prime(6));
        assert!(Solution::is_prime(7));
        assert!(!Solution::is_prime(8));
        assert!(!Solution::is_prime(9));
        assert!(!Solution::is_prime(10));
        assert!(Solution::is_prime(11));
        assert!(Solution::is_prime(97));
        assert!(Solution::is_prime(101));
        assert!(!Solution::is_prime(100));
    }
    
    #[test]
    fn test_boundary_conditions() {
        // Test with numbers exactly at boundary
        let matrix = vec![
            vec![1, 0],
            vec![1, 1]
        ];
        assert_eq!(Solution::most_frequent_prime(matrix), 11); // 11 is exactly > 10
        
        let matrix = vec![
            vec![1, 0],
            vec![1, 0]
        ];
        assert_eq!(Solution::most_frequent_prime(matrix), -1); // 10 is exactly = 10, not > 10
    }
    
    #[test]
    fn test_repeating_digits() {
        let matrix = vec![
            vec![1, 1],
            vec![1, 1]
        ];
        assert_eq!(Solution::most_frequent_prime(matrix), 11); // 11 can be formed multiple ways
        
        let matrix = vec![
            vec![3, 3],
            vec![3, 3]
        ];
        assert_eq!(Solution::most_frequent_prime(matrix), -1); // 33 is not prime
    }
    
    #[test]
    fn test_performance_with_large_matrix() {
        // Create a larger matrix to test performance
        let size = 10;
        let matrix: Vec<Vec<i32>> = (0..size)
            .map(|i| (0..size).map(|j| ((i*j) % 9) + 1).collect())
            .collect();
            
        let result = Solution::most_frequent_prime(matrix);
        assert!(result == -1 || (result > 10 && Solution::is_prime(result)));
    }
    
    #[test]
    fn test_matrix_with_zero() {
        let matrix = vec![
            vec![0, 1, 3],
            vec![1, 0, 1],
            vec![3, 1, 0]
        ];
        
        let result = Solution::most_frequent_prime(matrix);
        assert!(result == -1 || (result > 10 && Solution::is_prime(result)));
    }
    
    #[test]
    fn test_large_primes_at_edge() {
        let matrix = vec![
            vec![7, 9, 1, 9, 0, 1, 3],
            vec![2, 3, 7, 4, 1, 1, 1]
        ];
        
        let result = Solution::most_frequent_prime(matrix);
        assert!(result > 10 && Solution::is_prime(result));
    }
}
