fn is_solvable(
    target: i64,
    numbers: &Vec<i64>,
    curr_idx: usize,
    curr_num: i64,
    sequence: &mut Vec<char>,
) -> (bool, Vec<char>) {
    if curr_idx == numbers.len() {
        if curr_num == target {
            return (true, sequence.clone());
        } else {
            return (false, vec![]);
        }
    }

    let next_num = numbers[curr_idx];
    sequence.push('+');
    let is_plus_solv = is_solvable(target, numbers, curr_idx + 1, curr_num + next_num, sequence);
    sequence.pop();
    sequence.push('*');
    let is_mult_solv = is_solvable(target, numbers, curr_idx + 1, curr_num * next_num, sequence);
    sequence.pop();

    if is_plus_solv.0 {
        is_plus_solv
    } else if is_mult_solv.0 {
        is_mult_solv
    } else {
        (false, vec![])
    }
}

pub fn solve(input: &str) -> i64 {
    let mut sum = 0i64;
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let target = left.trim().parse::<i64>().unwrap();
        let operands = right
            .split(" ")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let is_solv = is_solvable(target, &operands, 1, operands[0], &mut vec![]);
        if is_solv.0 {
            sum += target;
            println!("Target: {}", target);
            println!("Operands: {:?}", operands);
            println!("Sequence: {:?}", is_solv.1);
        }
    }

    sum
}
