struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut mat = mat;
        for _ in 0..4 {
            if mat == target {
                return true;
            }
            Self::rotate_clockwise(&mut mat);

        }
        false
    }

    pub fn rotate_clockwise(mat: &mut Vec<Vec<i32>>) {
        let n = mat.len();
        for i in 0..n - 1 {
            for j in i..n {
                let tmp = mat[i][j];
                mat[i][j] = mat[j][i];
                mat[j][i] = tmp;
            }
        }

        mat.iter_mut().for_each(|r| r.reverse());
    }
}

fn main() {
    let mut rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", rows);
    Solution::rotate_clockwise(&mut rows);
    println!("{:?}", rows);
}
