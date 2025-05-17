pub struct Solution;
impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut sum = 0;
        for command in commands {
            match command.as_str() {
                "RIGHT" => sum += 1,
                "DOWN" => sum += n,
                "UP" => sum -= n,
                "LEFT" => sum -= 1,
                _ => {}
            }
        }

        sum
    }
}

fn main() {
    println!("Hello, world!");
}
