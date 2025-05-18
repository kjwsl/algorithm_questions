struct Solution;
impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match (z - x).abs().cmp(&(z-y).abs()) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => 2,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

fn main() {
    todo!()
}
