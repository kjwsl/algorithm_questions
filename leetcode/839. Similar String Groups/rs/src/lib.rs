pub struct Solution;

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        if n == 0 {
            return 0;
        }

        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut [usize], x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut [usize], x: usize, y: usize) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x != root_y {
                parent[root_x] = root_y;
            }
        }

        for i in 0..n {
            for j in i + 1..n {
                if Solution::is_similar(&strs[i], &strs[j]) {
                    union(&mut parent, i, j);
                }
            }
        }

        let mut count = 0;
        for i in 0..n {
            if parent[i] == i {
                count += 1;
            }
        }

        count
    }

    fn is_similar(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        let mut diff = 0;
        for i in 0..s1_bytes.len() {
            if s1_bytes[i] != s2_bytes[i] {
                diff += 1;
                if diff > 2 {
                    return false;
                }
            }
        }

        diff <= 2
    }
}
