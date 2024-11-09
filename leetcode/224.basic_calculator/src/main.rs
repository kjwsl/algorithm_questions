use std::vec::Vec;
mod test;
use test::Test;

// Define the Solution struct
struct Solution {}

// Define the Calculator struct with num_stack and op_stack using Vec for efficiency
struct Calculator {
    num_stack: Vec<i32>,
    op_stack: Vec<char>,
}

impl Calculator {
    /// Creates a new Calculator instance
    #[inline]
    pub fn new() -> Self {
        Self {
            num_stack: Vec::new(),
            op_stack: Vec::new(),
        }
    }

    /// Applies the top operator on the stacks to the top two numbers
    #[inline]
    fn apply_operator(&mut self) {
        // Ensure there are at least two numbers and one operator
        if let (Some(op), Some(num2), Some(num1)) = (
            self.op_stack.pop(),
            self.num_stack.pop(),
            self.num_stack.pop(),
        ) {
            let result = match op {
                '+' => num1 + num2,
                '-' => num1 - num2,
                _ => panic!("Unsupported operator: {}", op),
            };
            self.num_stack.push(result);
        }
    }

    /// Calculates the result of the given arithmetic expression
    pub fn calculate(&mut self, expression: &str) -> i32 {
        let mut chars = expression.chars().peekable();
        let mut prev_char: Option<char> = None;

        while let Some(&c) = chars.peek() {
            match c {
                ' ' => {
                    // Skip whitespace
                    chars.next();
                }
                '+' | '-' => {
                    // Determine if the operator is unary
                    if prev_char.is_none() || matches!(prev_char, Some('(') | Some('+') | Some('-'))
                    {
                        // Unary operator detected
                        if c == '-' {
                            // For unary '-', push 0 and '-' to op_stack to represent (0 - ...)
                            self.num_stack.push(0);
                            self.op_stack.push('-');
                        } else {
                            // For unary '+', push 0 and '+' to op_stack to represent (0 + ...)
                            self.num_stack.push(0);
                            self.op_stack.push('+');
                        }
                        chars.next(); // Consume the operator
                        prev_char = Some(c);
                    } else {
                        // Binary operator detected
                        // Apply all existing operators of equal or higher precedence
                        while let Some(&top_op) = self.op_stack.last() {
                            if top_op == '(' {
                                break;
                            }
                            self.apply_operator();
                        }
                        self.op_stack.push(c);
                        chars.next(); // Consume the operator
                        prev_char = Some(c);
                    }
                }
                '(' => {
                    // Push '(' to op_stack
                    self.op_stack.push('(');
                    chars.next(); // Consume '('
                    prev_char = Some('(');
                }
                ')' => {
                    // Apply all operators until '(' is encountered
                    while let Some(&op) = self.op_stack.last() {
                        if op == '(' {
                            break;
                        }
                        self.apply_operator();
                    }
                    if self.op_stack.last() == Some(&'(') {
                        self.op_stack.pop(); // Remove '(' from stack
                    } else {
                        panic!("Mismatched parentheses detected.");
                    }
                    chars.next(); // Consume ')'
                    prev_char = Some(')');
                }
                c if c.is_ascii_digit() => {
                    // Parse the number and push to num_stack
                    let mut num = 0;
                    while let Some(&c_digit) = chars.peek() {
                        if c_digit.is_ascii_digit() {
                            num = num * 10 + c_digit.to_digit(10).unwrap() as i32;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    self.num_stack.push(num);
                    prev_char = Some('n'); // 'n' denotes a number
                }
                _ => {
                    panic!("Invalid character encountered: {}", c);
                }
            }
        }

        // Apply any remaining operators
        while let Some(&op) = self.op_stack.last() {
            if op == '(' {
                panic!("Mismatched parentheses detected.");
            }
            self.apply_operator();
        }

        // The final result should be the only number left on the stack
        self.num_stack.pop().unwrap_or(0)
    }
}

impl Solution {
    /// Public method to calculate the expression
    pub fn calculate(s: String) -> i32 {
        let mut calculator = Calculator::new();
        calculator.calculate(&s)
    }
}

fn main() {
    let mut test = Test::new();
    test.validate(Solution::calculate(String::from("1 + 1")), 2);
    test.validate(Solution::calculate(String::from(" 2-1 + 2 ")), 3);
    test.validate(Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
    test.validate(Solution::calculate(String::from("1-(     -2)")), 3);
    test.validate(Solution::calculate(String::from("-2+ 1")), -1);
    test.validate(Solution::calculate(String::from("-(3+(2-1))")), -4);
}
