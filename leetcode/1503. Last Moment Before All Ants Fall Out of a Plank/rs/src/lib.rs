pub struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        // 4,3 0,1
        // 2,3 1,2
        // 1,2 2,3
        // 0,1 3,4
        // 0 4

        if left.is_empty() {
            return right.len() as i32;
        } else if right.is_empty() {
            return left.len() as i32;
        }

        let mut left = left;
        let mut rigth = right;

        let mut t = 0;  

        while !left.is_empty() && !right.is_empty() {

            
        }
        

        
    }
}
