#[derive(Clone, Debug)]
enum Operation {
    Plus,
    Mult,
    Concat,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Plus => a + b,
            Operation::Mult => a * b,
            Operation::Concat => {
                let digit = b.to_string().len() as u32;
                a * 10i64.pow(digit) + b
            }
        }
    }
}

fn is_solvable(
    target: i64,
    numbers: &Vec<i64>,
    curr_idx: usize,
    curr_num: i64,
    sequence: &mut Vec<Operation>,
    possible_ops: &Vec<Operation>,
) -> (bool, Vec<Operation>) {
    if curr_idx == numbers.len() {
        if curr_num == target {
            return (true, sequence.clone());
        } else {
            return (false, vec![]);
        }
    }

    let next_num = numbers[curr_idx];

    let mut is_solv = (false, vec![]);
    for op in possible_ops {
        sequence.push(op.clone());
        is_solv = is_solvable(
            target,
            numbers,
            curr_idx + 1,
            op.apply(curr_num, next_num),
            sequence,
            possible_ops,
        );
        sequence.pop();
        if is_solv.0 {
            break;
        }
    }

    is_solv
}

fn to_equation(target: i64, operands: &[i64], sequence: &[Operation]) -> String {
    let mut equation = String::new();
    equation.push_str(&operands[0].to_string());
    for (i, op) in sequence.iter().enumerate() {
        equation.push(' ');
        match op {
            Operation::Plus => equation.push('+'),
            Operation::Mult => equation.push('*'),
            Operation::Concat => equation.push_str("||"),
        }
        equation.push(' ');
        equation.push_str(&operands[i + 1].to_string());
    }
    equation.push_str(" = ");
    equation.push_str(&target.to_string());
    equation
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

        let possible_ops = vec![Operation::Plus, Operation::Mult, Operation::Concat];
        let is_solv = is_solvable(
            target,
            &operands,
            1,
            operands[0],
            &mut vec![],
            &possible_ops,
        );
        if is_solv.0 {
            sum += target;
            println!("{}", to_equation(target, &operands, &is_solv.1));
        }
    }

    sum
}
