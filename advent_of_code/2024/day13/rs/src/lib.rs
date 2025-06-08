pub struct Solution;

impl Solution {
    fn parse_input(input: &str) -> Vec<ClawMachine> {
        let mut ret = vec![];
        for w in input.lines().collect::<Vec<_>>().chunks_exact(4) {
            let button_a_str = w[0];
            let button_b_str = w[1];
            let prize_str = w[2];

            let (a_x, a_y) = button_a_str[10..].split_once(", ").unwrap();
            let a_x = a_x[2..].parse::<i64>().unwrap();
            let a_y = a_y[2..].parse::<i64>().unwrap();

            let (b_x, b_y) = button_b_str[10..].split_once(", ").unwrap();
            let b_x = b_x[2..].parse::<i64>().unwrap();
            let b_y = b_y[2..].parse::<i64>().unwrap();

            let (prize_x, prize_y) = prize_str[7..].split_once(", ").unwrap();
            let prize_x = prize_x[2..].parse::<i64>().unwrap();
            let prize_y = prize_y[2..].parse::<i64>().unwrap();

            let machine = ClawMachine::new((a_x, a_y), (b_x, b_y), (prize_x, prize_y));
            ret.push(machine);
        }

        ret
    }
    pub fn part1(input: &str) -> i64 {
        let machines = Self::parse_input(input);
        machines
            .iter()
            .filter_map(|machine| machine.min_cost_to_win())
            .sum()
    }
}

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    pub fn new(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    pub fn min_cost_to_win(&self) -> Option<i64> {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        let determinant = ax * by - bx * ay;

        if determinant == 0 {
            return None; // The buttons are collinear, no unique solution
        }

        let a_presses = px * by - py * bx;
        let b_presses = ax * py - ay * px;

        if a_presses % determinant != 0 || b_presses % determinant != 0 {
            return None; // No integer solution
        }

        let a = a_presses / determinant;
        let b = b_presses / determinant;

        if a >= 0 && b >= 0 && a <= 100 && b <= 100 {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}
