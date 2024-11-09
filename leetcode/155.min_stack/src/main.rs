/// https://leetcode.com/problems/min-stack/?envType=study-plan-v2&envId=top-interview-150
mod test;
use test::Test;

struct MinStack {
    vals: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self { vals: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.vals.push(val);
    }

    fn pop(&mut self) {
        self.vals.pop();
    }

    fn get_min(&self) -> i32 {
        *self.vals.iter().min().unwrap()
    }

    fn top(&self) -> i32 {
        *self.vals.last().unwrap()
    }
}

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
    println!("All tests passed!");
}
