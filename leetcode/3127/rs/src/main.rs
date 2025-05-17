struct Solution;
impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let mut chs = [
                    grid[i][j],
                    grid[i][j + 1],
                    grid[i + 1][j],
                    grid[i + 1][j + 1],
                ];

                let blacks = chs.iter().filter(|&&ch| ch == 'B').count();
                if blacks < 5 && blacks != 2 {
                    return true;
                }
            }
        }
        false
    }
}
fn main() {
    println!("Hello, world!");
}
