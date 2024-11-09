use std::collections::HashMap;

fn solve(s: String) -> bool {
    let parentheses: std::collections::HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')]
        .iter()
        .cloned()
        .collect();
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            _ => {
                if stack.is_empty() {
                    return false;
                }
                let last = stack.pop().unwrap();
                if last != parentheses[&c] {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

fn test(s: &str, expect: bool) {
    static mut TEST_NUM: i32 = 0;
    let result = solve(s.to_string());
    let success = result == expect;
    println!(
        "Test #{}: {}",
        unsafe { TEST_NUM },
        if success { "Success" } else { "Failed" }
    );
    if !success {
        println!("  expect: {}", expect);
        println!("  result: {}", result);
    }
    unsafe {
        TEST_NUM += 1;
    }
}

fn main() {
    test("()", true);
    test("()[]{}", true);
    test("(]", false);
    test("([)]", false);
    test("{[]}", true);
    test("(", false);
    test(")", false);
}
