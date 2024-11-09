mod test;
use std::collections::VecDeque;
use test::Test;

struct Solution {}

struct Calculator {
    num_stack: VecDeque<i32>,
    op_stack: VecDeque<String>,
}

trait BinaryOperation<T, Output>: Fn(T, T) -> Output {}

impl<T, Output, F> BinaryOperation<T, Output> for F where F: Fn(T, T) -> Output {}

impl Calculator {
    pub fn new() -> Self {
        Self {
            num_stack: VecDeque::new(),
            op_stack: VecDeque::new(),
        }
    }

    fn operate(&mut self) {
        println!("Operating binary...");
        let op = self.op_stack.pop_front().unwrap();
        let val = match op.as_str() {
            "+" => {
                let num1 = self.num_stack.pop_front().unwrap();
                let num2 = self.num_stack.pop_front().unwrap();
                num1 + num2
            }
            "-" => num1 - num2,
            _ => num1,
        };
        println!("{} {} {} = {}", num1, op, num2, val);
        self.num_stack.push_front(val);
    }

    fn operate_binary_priority(&mut self) {
        let mut num_stack = Vec::new();
        let mut op_stack = Vec::new();
        num_stack.push(self.num_stack.pop_back().unwrap());
        while !self.op_stack.is_empty() {
            let op: String = self.op_stack.pop_back().unwrap();
            if op.as_str() == "(" {
                break;
            }
            op_stack.push(op);
            num_stack.push(self.num_stack.pop_back().unwrap());
        }

        while !op_stack.is_empty() {
            let num1 = num_stack.pop().unwrap();
            let num2 = num_stack.pop().unwrap();
            let op = op_stack.pop().unwrap();
            let val = match op.as_str() {
                "+" => num1 + num2,
                "-" => num1 - num2,
                _ => num1,
            };
            num_stack.push(val);
        }

        self.num_stack.push_back(num_stack.pop().unwrap());
    }

    pub fn calculate(&mut self, expression: &str) -> i32 {
        let mut paren_cnt = 0;
        let mut it = expression.chars().peekable();
        while let Some(c) = it.peek() {
            match c {
                ' ' => {
                    it.next();
                }
                '+' | '-' => {
                    self.op_stack.push_back(c.to_string());
                    it.next();
                }
                '(' => {
                    paren_cnt += 1;
                    it.next();
                }
                ')' => {
                    while !self.op_stack.is_empty() {
                        self.operate_binary_priority();
                    }
                    paren_cnt -= 1;
                    it.next();
                }
                _ => {
                    let mut num = 0;
                    while let Some(c) = it.peek() {
                        if c.is_ascii_digit() {
                            num = num * 10 + c.to_digit(10).unwrap() as i32;
                            it.next();
                        } else {
                            break;
                        }
                    }
                    self.num_stack.push_back(num);
                }
            }
        }

        while !self.op_stack.is_empty() {
            self.operate();
        }

        self.num_stack.pop_front().unwrap()
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut calculator = Calculator::new();

        calculator.calculate(s.as_str())
    }
}

fn main() {
    let mut test = Test::new();
    test.validate(Solution::calculate(String::from("1 + 1")), 2);
    test.validate(Solution::calculate(String::from(" 2-1 + 2 ")), 3);
    test.validate(Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
    test.validate(Solution::calculate(String::from("1-(     -2)")), 3);
}
