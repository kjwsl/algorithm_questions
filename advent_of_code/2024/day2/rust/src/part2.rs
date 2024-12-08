fn check_safety(level_a: i32, level_b: i32, prev_diff: Option<i32>) -> bool {
    let diff = level_b - level_a;
    if diff == 0 || diff.abs() > 3 {
        return false;
    }
    if let Some(prev_diff) = prev_diff {
        if prev_diff.cmp(&0) != diff.cmp(&0) {
            return false;
        }
    }
    true
}

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

        let mut vulnerability_count = 0;
        let mut prev_diff = None;
        const TOLERANCE: i32 = 1;

        for x in levels.windows(2) {
            let (a, b) = (x[0], x[1]);
            if !check_safety(a, b, prev_diff) {
                vulnerability_count += 1;
            }
            if vulnerability_count > TOLERANCE {
                break;
            }
            prev_diff = Some(b - a);
        }

        if vulnerability_count <= TOLERANCE {
            count += 1;
        }
    }
    count
}
