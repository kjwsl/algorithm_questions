mod test;
use test::Test;

fn solve(points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }

    let mut points = points;
    points.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut cnt = 1;
    let mut i = 1;
    let mut smallest = points[0][0];
    let mut biggest = points[0][1];

    while i < points.len() {
        smallest = smallest.max(points[i][0]);
        biggest = biggest.min(points[i][1]);
        if smallest > biggest {
            cnt += 1;
            smallest = points[i][0];
            biggest = points[i][1];
        }
        i += 1;
    }

    cnt
}

fn solve2(points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }

    // Sort intervals by their start points
    let mut points = points;
    points.sort_by_key(|point| point[0]);

    let mut arrow_count = 1;
    let mut current_end = points[0][1];

    for point in &points[1..] {
        let start = point[0];
        let end = point[1];

        // If current balloon starts after the last tracked end, increment arrow count
        if start > current_end {
            arrow_count += 1;
            current_end = end;
        } else {
            // Otherwise, overlap exists, so update the end to be the minimum end
            current_end = current_end.min(end);
        }
    }

    arrow_count
}

fn main() {
    let mut test = Test::new();
    test.validate(
        solve(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2,
    );
    test.validate(
        solve(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
        4,
    );
    test.validate(
        solve(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
        2,
    );
}
