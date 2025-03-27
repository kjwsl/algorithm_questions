pub struct Solution;

// 1) 1
// 2) 2 | 1 + 1
// 3) 5 | 2 + 1 + 2
// 4) 14 | 5 + 2 + 2 + 5 
// 5) 42 | 14 + 5 + 2 + 2 + 5 + 14 | 14 + 14 + 14
// 6) 132  | 42 + 14 + ( 2+5 ) + (5+2) + 14 + 42 | 42 + 48 + 42

//0, 1, 4, 24, 48

// 1 3 12 60 74
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n <= 1 {
            1
        } else {
            Self::num_trees(n - 1) * 2 + Self::num_trees(n - 2)
                3 4 5 
        }
    }
}
