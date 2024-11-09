/// https://leetcode.com/problems/evaluate-reverse-polish-notation/description/?envType=study-plan-v2&envId=top-interview-150
mod test;
use test::Test;

fn to_num(txt: Option<String>, default: i32) -> i32 {
    match txt {
        Some(txt) => {
            let number = txt.parse::<i32>();
            number.unwrap_or(default)
        }
        None => default,
    }
}

fn solve(tokens: Vec<&str>) -> i32 {
    let mut stack = Vec::<String>::new();

    for token in tokens {
        match token {
            "+" => {
                if stack.len() >= 2 {
                    let a = to_num(stack.pop(), 0);
                    let b = to_num(stack.pop(), 0);
                    stack.push((a + b).to_string());
                }
            }
            "-" => {
                if stack.len() >= 2 {
                    let a = to_num(stack.pop(), 0);
                    let b = to_num(stack.pop(), 0);
                    stack.push((b - a).to_string());
                }
            }
            "*" => {
                if stack.len() >= 2 {
                    let a = to_num(stack.pop(), 1);
                    let b = to_num(stack.pop(), 1);
                    stack.push((b * a).to_string());
                }
            }
            "/" => {
                if stack.len() >= 2 {
                    let a = to_num(stack.pop(), 1);
                    let b = to_num(stack.pop(), 1);
                    stack.push((b / a).to_string());
                }
            }
            text => {
                if text.parse::<i32>().is_ok() {
                    stack.push(text.to_string());
                }
            }
        }
    }

    to_num(stack.pop(), 0)
}

fn main() {
    let mut test = Test::new();
    test.validate(solve(vec!["2", "1", "+", "3", "*"]), 9);
    test.validate(solve(vec!["4", "13", "5", "/", "+"]), 6);
    test.validate(
        solve(vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]),
        22,
    );
}
