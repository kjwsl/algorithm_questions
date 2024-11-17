pub struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix.iter().flatten().any(|x| x == &target)
    }
}
