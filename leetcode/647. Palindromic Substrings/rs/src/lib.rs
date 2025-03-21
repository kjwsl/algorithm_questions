pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut count = 0;
        
        for i in 0..n {
            // Count odd-length palindromes (centered at i)
            count += Self::expand_around_center(s_bytes, i as isize, i as isize);
            
            // Count even-length palindromes (centered between i and i+1)
            if i + 1 < n {
                count += Self::expand_around_center(s_bytes, i as isize, (i + 1) as isize);
            }
        }
        
        count
    }
    
    #[inline]
    fn expand_around_center(s: &[u8], mut left: isize, mut right: isize) -> i32 {
        let n = s.len() as isize;
        let mut count = 0;
        
        while left >= 0 && right < n && s[left as usize] == s[right as usize] {
            count += 1;
            left -= 1;
            right += 1;
        }
        
        count
    }
}
