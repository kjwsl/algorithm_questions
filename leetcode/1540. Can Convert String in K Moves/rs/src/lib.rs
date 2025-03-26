pub struct Solution;

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut idx = 0;
        let mut map = std::collections::HashMap::new();

        s.as_bytes().iter().enumerate().for_each(|(i, c)| {
            map.insert(c, i);
        });

        for i in 1..=k {
            while idx < t.len() && s.as_bytes()[idx] == t.as_bytes()[idx] {
                idx += 1;
            }
            if idx >= t.len() {
                return true;
            }

            let source = s.as_bytes()[idx];
            let target = t.as_bytes()[idx];


            if map.contains_key(&target) {
                map.remove(&target);
                idx += 1;
            } else if source + i as u8 == target {
                idx += 1;
            }
        }
        println!("idx: {}", idx);

        idx == t.len()
    }
}
