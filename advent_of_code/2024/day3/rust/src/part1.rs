pub fn solve(input: &str) -> i32 {
    let mut result = 0;
    let mut input = input;
    while let Some(idx) = input.find("mul(") {
        input = &input[idx + 4..];
        if let Some(end_idx) = input[..8].find(")") {
            let arg_field = &input[..end_idx];
            if (3..=7).contains(&arg_field.len()) {
                if let Some(comma_idx) = arg_field.find(",") {
                    if let (Ok(arg1), Ok(arg2)) = (
                        arg_field[..comma_idx].trim().parse::<i32>(),
                        arg_field[comma_idx + 1..].trim().parse::<i32>(),
                    ) {
                        result += arg1 * arg2;
                    }
                }
            }
        }
    }
    result
}
