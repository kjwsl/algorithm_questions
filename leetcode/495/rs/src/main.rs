pub struct Solution;
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.len() == 1 {
            return duration;
        }
        let mut elapsed = duration;
        let mut last_time = time_series[0] + duration;
        for &t in time_series.iter().skip(1) {
            let new_last_time = t + duration;
            if new_last_time <= last_time {
                continue;
            }

            elapsed += new_last_time - t.max(last_time);
            last_time = new_last_time;
        }

        elapsed
    }
}
fn main() {
    println!("Hello, world!");
}
