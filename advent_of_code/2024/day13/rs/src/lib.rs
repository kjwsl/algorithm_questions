pub struct Solution;

struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

impl Machine {
    pub fn new(button_a: (i32, i32), button_b: (i32, i32), prize: (i32, i32)) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

impl Solution {

    pub fn part1(input: &str) -> {
        let machines = Self::parse_input(input);
        for machine in machines {
        }
    }
    pub fn parse_input(
        input: &str,
    ) -> Vec<Machine> {
        let mut lines = input.lines().collect::<Vec<_>>();
        let mut machines = vec![];
        for chunk in lines.windows(4) {
            let button_a_str = chunk[0];
            let button_b_str = chunk[1];
            let prize_str = chunk[2];
            let mut button_a = None;
            let mut button_b = None;
            let mut prize = None;

            for (i, button_str) in chunk[0..2].iter().enumerate() {
                let x_idx = button_str.find("X").unwrap();
                let y_idx = button_str.find("Y").unwrap();
                let x = button_str[x_idx + 2..y_idx - 2].parse::<i32>().unwrap()
                    * if button_str.as_bytes()[x_idx + 1] == b'-' {
                        -1
                    } else {
                        1
                    };
                let y = button_str[y_idx + 2..].trim().parse::<i32>().unwrap()
                    * if button_str.as_bytes()[y_idx + 1] == b'-' {
                        -1
                    } else {
                        1
                    };
                let res = (x, y);
                if i == 0 {
                    button_a = Some(res);
                } else {
                    button_b = Some(res);
                }
            }
            let comma_idx = prize_str.find(",").unwrap();
            let x = prize_str[9..comma_idx].parse::<i32>().unwrap();
            let y = prize_str[comma_idx + 4..].parse::<i32>().unwrap();
            prize = Some((x, y));
            let machine = Machine::new(button_a.unwrap(), button_b.unwrap(), prize.unwrap());
            machines.push(machine);
        }

        machines
    }
}

// a*x + b*y = c
// 3x + y = cost
