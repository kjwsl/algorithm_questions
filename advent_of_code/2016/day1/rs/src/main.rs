#[derive(Debug, Clone)]
struct Instruction {
    direction: char,
    distance: i32,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut i = 0;

    let mut ret = vec![];

    while i < input.len() {
        let direction = input.chars().nth(i).unwrap();
        i += 1;

        let mut comma_idx = i;

        while comma_idx < input.len() && input.chars().nth(comma_idx).unwrap() != ',' {
            comma_idx += 1;
        }
        dbg!(i);
        dbg!(comma_idx);

        let distance: i32 = match input[i..comma_idx].trim_end().parse() {
            Ok(d) => d,
            Err(_) => panic!("Failed to parse {}", &input[i..comma_idx]),
        };

        let inst = Instruction {
            direction,
            distance,
        };
        println!("{:?}", &inst);
        ret.push(inst);

        i = comma_idx + 2;
    }

    ret
}
fn part1(input: &str) -> i32 {
    let instructions = parse_input(input);

    let mut x = 0;
    let mut y = 0;

    const DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];

    let mut current_direction = 0;

    for inst in instructions {
        current_direction = match inst.direction {
            'R' => (current_direction + 1) % 4,
            'L' => (current_direction + 3) % 4,
            _ => unreachable!(),
        };

        match DIRECTIONS[current_direction] {
            'N' => y += inst.distance,
            'S' => y -= inst.distance,
            'E' => x += inst.distance,
            'W' => x -= inst.distance,
            _ => unreachable!(),
        }
    }

    x.abs() + y.abs()
}

fn main() {
    let sample = std::fs::read_to_string("../sample.txt").unwrap();

    println!("Part 1: {}", part1(&sample));
}
