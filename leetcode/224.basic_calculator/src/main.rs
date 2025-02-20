use std::vec::Vec;
mod test;
use test::Test;

#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Operator(Operator),
    LeftParen,
    RightParen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Add,
    Subtract,
}

impl Operator {
    fn apply(&self, left: i32, right: i32) -> i32 {
        match self {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
        }
    }
}

#[derive(Debug)]
struct Calculator {
    numbers: Vec<i32>,
    operators: Vec<(Operator, u8)>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            numbers: Vec::new(),
            operators: Vec::new(),
        }
    }

    fn evaluate_top(&mut self) {
        if let (Some(right), Some(left)) = (self.numbers.pop(), self.numbers.pop()) {
            if let Some((op, _)) = self.operators.pop() {
                let result = op.apply(left, right);
                self.numbers.push(result);
            }
        }
    }

    fn evaluate(&mut self, tokens: Vec<Token>) -> Result<i32, String> {
        for token in tokens {
            match token {
                Token::Number(n) => self.numbers.push(n),
                Token::Operator(op) => {
                    while !self.operators.is_empty() 
                        && self.operators.last().unwrap().1 >= 1 {
                        self.evaluate_top();
                    }
                    self.operators.push((op, 1));
                }
                Token::LeftParen => self.operators.push((Operator::Add, 0)),
                Token::RightParen => {
                    while !self.operators.is_empty() && self.operators.last().unwrap().1 > 0 {
                        self.evaluate_top();
                    }
                    self.operators.pop().ok_or("Mismatched parentheses")?;
                }
            }
        }

        while !self.operators.is_empty() {
            if self.operators.last().unwrap().1 == 0 {
                return Err("Mismatched parentheses".to_string());
            }
            self.evaluate_top();
        }

        self.numbers.pop().ok_or("Empty expression".to_string())
    }
}

struct Parser;

impl Parser {
    fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = expr.chars().peekable();
        let mut last_token: Option<&Token> = None;

        while let Some(&c) = chars.peek() {
            match c {
                ' ' => { chars.next(); }
                '0'..='9' => {
                    let mut number = 0;
                    while let Some(&d) = chars.peek() {
                        if let Some(digit) = d.to_digit(10) {
                            number = number * 10 + digit as i32;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number));
                }
                '+' | '-' => {
                    let is_unary = last_token.map_or(true, |t| 
                        matches!(t, Token::Operator(_) | Token::LeftParen) || last_token.is_none());
                    chars.next();
                    
                    if is_unary && c == '-' {
                        tokens.push(Token::Number(0));
                        tokens.push(Token::Operator(Operator::Subtract));
                    } else if !is_unary {
                        tokens.push(Token::Operator(
                            if c == '+' { Operator::Add } else { Operator::Subtract }
                        ));
                    }
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RightParen);
                }
                _ => return Err(format!("Invalid character: {}", c)),
            }
            last_token = tokens.last();
        }
        Ok(tokens)
    }
}

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let tokens = Parser::tokenize(&s).unwrap_or_default();
        let mut calculator = Calculator::new();
        calculator.evaluate(tokens).unwrap_or(0)
    }
}

fn main() {
    let mut test = Test::new();
    test.validate(Solution::calculate(String::from("1 + 1")), 2);
    test.validate(Solution::calculate(String::from(" 2-1 + 2 ")), 3);
    test.validate(Solution::calculate(String::from("-2+ 1")), -1);
    test.validate(Solution::calculate(String::from("(1 + 1)")), 2);
    test.validate(Solution::calculate(String::from("2-(1 + 2)")), -1);
    test.validate(Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
}
