pub fn solve(input: &str) -> i32 {
    let mut result = 0;
    let mut input = input;
    let mut is_enabled = true;
    while let Some(idx) = input.find("mul(") {
        let mut prev = &input[..idx];
        while let Some((do_idx, dont_idx)) = prev.find("do()").zip(prev.find("don't()")) {
            if do_idx < dont_idx {
                is_enabled = false;
                prev = &prev[do_idx + 4..];
            } else {
                is_enabled = true;
                prev = &prev[dont_idx + 7..];
            }
        }
        if let Some(do_idx) = prev.find("do()") {
            is_enabled = true;
            prev = &prev[do_idx + 4..];
        } else if let Some(dont_idx) = prev.find("don't()") {
            is_enabled = false;
            prev = &prev[dont_idx + 7..];
        }
        input = &input[idx + 4..];
        if !is_enabled {
            continue;
        }
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
