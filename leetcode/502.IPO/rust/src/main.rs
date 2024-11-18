struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<(i32, i32)> = profits
            .iter()
            .zip(capital.iter())
            .map(|(p, c)| (*p, *c))
            .collect();
        projects.sort_by(|a, b| a.1.cmp(&b.1));

        let mut heap = std::collections::BinaryHeap::new();
        let mut w = w;
        let mut i = 0;

        for _ in 0..k {
            while i < projects.len() && projects[i].1 <= w {
                heap.push(projects[i].0);
                i += 1;
            }

            if let Some(p) = heap.pop() {
                w += p;
            } else {
                break;
            }
        }

        w
    }
}
