pub fn solve(input: &str) -> String {
    let mut input = input;
    let mut new_input = String::new();
    while let Some(idx) = new_input.find("mul(") {
        input = &input[idx..];
        if let Some(end_idx) = input.find(")") {
            let instruction = &input[..end_idx + 1];
            let mut numbers = instruction.split(",").collect::<Vec<_>>();
            if numbers.len() != 2 {
                continue;
            }
            let a = numbers[0].trim().parse::<i32>().unwrap();
            let b = numbers[1].trim().parse::<i32>().unwrap();

        } else {
            break;
        }
    }

    new_input
}
