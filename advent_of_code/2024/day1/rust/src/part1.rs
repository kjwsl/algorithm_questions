pub fn solve(input: &str) -> i32 {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once(" ").expect("No way");
        let left = left.trim().parse::<i32>().expect("left is not a number");
        let right = right.trim().parse::<i32>().expect("right is not a number");
        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .iter()
        .zip(right_list.iter())
        .fold(0, |acc, (left, right)| (left - right).abs() + acc)
}
