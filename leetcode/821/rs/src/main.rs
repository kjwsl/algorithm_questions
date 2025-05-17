pub struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let idxs = s
            .match_indices(|ch| ch == c )
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        let mut res = vec![];

        for (i, _) in s.chars().enumerate() {
            let dis = idxs.iter().map(|idx| (*idx as i32 - i as i32).abs()).min().unwrap();
            res.push(dis);
        }

        res
    }
}

fn main() {
    println!("Hello, world!");
}
