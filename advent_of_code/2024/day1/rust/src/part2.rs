pub fn solve(input: &str) -> i32 {
    let mut simplicity_score = 0;
    let mut left_list = Vec::new();
    let mut right_hash = std::collections::HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" ").expect("No way");
        let left = left.trim().parse::<i32>().expect("left is not a number");
        let right = right.trim().parse::<i32>().expect("right is not a number");

        left_list.push(left);

        match right_hash.get_mut(&right) {
            Some(count) => *count += 1,
            None => {
                right_hash.insert(right, 1);
            }
        }
    }

    for val in left_list {
        simplicity_score += match right_hash.get(&val) {
            Some(count) => count * val,
            None => 0,
        }
    }

    simplicity_score
}
