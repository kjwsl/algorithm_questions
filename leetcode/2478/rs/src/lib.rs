pub struct Solution;

impl Solution {
    const PRIME_DIGITS: [char; 4] = ['2', '3', '5', '7'];
    const MOD: i32 = 1_000_000_007;
    
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let min_len = min_length as usize;
        
        // Early validation checks
        if !Self::is_prime(s_chars[0]) || Self::is_prime(s_chars[n-1]) || 
           k as usize > n || min_len * k as usize > n {
            return 0;
        }
        
        // Find all valid partition points
        let mut valid_cuts = vec![0]; // Starting point
        for i in 1..n {
            if !Self::is_prime(s_chars[i-1]) && Self::is_prime(s_chars[i]) {
                valid_cuts.push(i);
            }
        }
        valid_cuts.push(n); // End point
        
        // Optimization: If we don't have enough valid cuts, return early
        if valid_cuts.len() < k as usize + 1 {
            return 0;
        }
        
        // dp[i][j] = ways to form j partitions ending at valid_cuts[i]
        let mut dp = vec![vec![0; k as usize + 1]; valid_cuts.len()];
        dp[0][0] = 1; // Base case: 0 partitions at starting point
        
        for j in 1..=k as usize {
            // Using prefix sum to avoid repeating inner loop calculations
            let mut prefix_sum = vec![0; valid_cuts.len()];
            
            for i in 0..valid_cuts.len() {
                if j > 1 || i == 0 {
                    prefix_sum[i] = dp[i][j-1];
                }
                if i > 0 {
                    prefix_sum[i] = (prefix_sum[i] + prefix_sum[i-1]) % Self::MOD;
                }
            }
            
            for i in 1..valid_cuts.len() {
                for prev in 0..i {
                    if valid_cuts[i] - valid_cuts[prev] >= min_len {
                        dp[i][j] = (dp[i][j] + dp[prev][j-1]) % Self::MOD;
                    } else if prev > 0 {
                        // Can't meet minimum length requirement with any further previous cuts
                        break;
                    }
                }
            }
        }
        
        dp[valid_cuts.len()-1][k as usize]
    }
    
    #[inline]
    fn is_prime(c: char) -> bool {
        matches!(c, '2' | '3' | '5' | '7')
    }
}
