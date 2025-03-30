
pub struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn k_similarity(a: String, b: String) -> i32 {
        let mut a = a.as_bytes();
        let mut b = b.as_bytes();
        let mut deq = VecDeque::new();
        let mut visited = HashMap::new();
        deq.push_back((a.to_vec(), 0));
        visited.insert(a.to_vec(), 0);

        while !deq.is_empty() {
            let (cur, step) = deq.pop_front().unwrap();
            if cur == b {
                return step;
            }
            let mut i = 0;
            while i < cur.len() && cur[i] == b[i] {
                i += 1;
            }
            for j in i + 1..cur.len() {
                if cur[j] == b[i] && cur[j] != b[j] {
                    let mut next = cur.clone();
                    next.swap(i, j);
                    if !visited.contains_key(&next) {
                        deq.push_back((next.clone(), step + 1));
                        visited.insert(next, step + 1);
                    }
                }
            }
        }
        
        -1
    }
}
fn main() {
    println!("Hello, world!");
}
