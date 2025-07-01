fn main() {
    let ans = Solution::min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]);
    println!("Answer: {}", ans);
}

struct Solution;

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        // 2^(i+1) - 1 = n
        assert!(
            ((n + 1) as f64).log2().fract() == 0.0,
            "n must be of the form 2^k - 1"
        );

        if n == 1 {
            return cost[0];
        }

        let mut cost = cost;

        let depth = (((n + 1) as f64).log2() as u32)
            .checked_sub(1)
            .expect("Failed to get depth");
        let num_leaf_nodes = 2u32.pow(depth) as usize;

        if cost.len() < num_leaf_nodes {
            panic!("No way!!!!");
        }

        let mut target = 1;
        for i in 0..cost.len() - num_leaf_nodes {
            cost[target] += cost[i];
            cost[target + 1] += cost[i];
            target += 2;
        }

        let costs = &cost[cost.len() - num_leaf_nodes..];
        let max_cost = *costs.iter().max().unwrap();
        let costs_d: Vec<_> = costs.iter().map(|c| max_cost - c).collect();

        Self::min_increment_from_costs(&costs_d) as i32
    }

    fn min_increment_from_costs(costs_d: &[i32]) -> usize {
        assert!(costs_d.len() % 2 == 0 || costs_d.len() == 1);
        if costs_d.len() == 1 {
            return costs_d[0] as usize;
        }

        let common_costs = *costs_d.iter().min().unwrap();
        let costs_d = costs_d.iter().map(|c| c - common_costs).collect::<Vec<_>>();
        let mid = costs_d.len() / 2;

        common_costs as usize
            + Self::min_increment_from_costs(&costs_d[..mid])
            + Self::min_increment_from_costs(&costs_d[mid..])
    }
}
