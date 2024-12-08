pub fn solve(input: &str) -> i32 {
    let reports = input.lines().collect::<Vec<_>>();
    let mut count = 0;

    for report in reports {
        let levels = report
            .split_whitespace()
            .map(|x| {
                x.trim()
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("Not a number: {}", x))
            })
            .collect::<Vec<_>>();

        let mut is_safe = true;
        let mut diff = 0;
        for x in levels.windows(2) {
            let (a, b) = (x[0], x[1]);
            let new_diff = b - a;
            if new_diff == 0 || new_diff.abs() > 3 {
                is_safe = false;
                break;
            }

            // If this is the first time the difference is not 0
            if diff == 0 {
                diff = new_diff;
                continue;
            }

            // If the sign of the difference changes
            if diff.cmp(&0) != new_diff.cmp(&0) {
                is_safe = false;
                break;
            }

            if new_diff.abs() > 3 {
                is_safe = false;
                break;
            }
            diff = new_diff;
        }

        if diff == 0 || diff.abs() > 3 {
            is_safe = false;
        }

        if is_safe {
            count += 1;
        }
    }
    count
}
