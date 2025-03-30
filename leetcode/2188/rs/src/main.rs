pub struct Solution;

fn eq(a: i32, b: i32) {
    let is_eq = a == b;
    println!("{} {} {}", a, if is_eq { "==" } else { "!=" }, b);
}

impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let num_laps = num_laps as usize;
        let max_consecutive = 20.min(num_laps); // Practical limit to avoid overflow
        
        // best_times[i] = minimum time to complete i consecutive laps with the same tire
        let mut best_times = vec![i32::MAX; max_consecutive + 1];
        best_times[0] = 0;
        
        for tire in &tires {
            let f = tire[0];
            let r = tire[1];
            
            let mut time = 0;
            let mut lap_time = f;
            
            // Use iterator-based approach to address the clippy warning
            for i in 1..=max_consecutive {
                time += lap_time;
                best_times[i] = best_times[i].min(time);
                
                // Early stopping: if current lap time is already worse than changing tires
                if lap_time >= f + change_time {
                    break;
                }
                
                // Check for potential overflow before updating lap_time
                if r > 1 && lap_time > i32::MAX / r {
                    break;
                }
                lap_time *= r;
            }
        }
        
        // dp[i] = minimum time to complete i laps with optimal tire changes
        let mut dp = vec![i32::MAX; num_laps + 1];
        dp[0] = 0;
        
        for i in 1..=num_laps {
            for j in 1..=max_consecutive.min(i) {
                // Skip invalid states or states that can't improve the solution
                if dp[i-j] == i32::MAX || best_times[j] == i32::MAX {
                    continue;
                }
                
                // Simplify the conditional logic
                let change_penalty = if i > j { change_time } else { 0 };
                dp[i] = dp[i].min(dp[i-j] + best_times[j] + change_penalty);
            }
        }
        
        dp[num_laps]
    }
}


fn main() {
    let tires = vec![vec![2, 3], vec![3, 4]];
    eq(Solution::minimum_finish_time(tires, 5, 4), 21);

    let tires = vec![vec![1, 10], vec![2, 2], vec![3, 4]];
    eq(Solution::minimum_finish_time(tires, 6, 5), 25);
}
