pub fn solve(input: &str) -> i32 {
    let lines = input.lines();

    let mut max = 0;

    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            max = max.max(current);
            current = 0;
            continue;
        }

        current += line
            .parse::<i32>()
            .expect("The line doesn't contain a number");
    }

    max
}
