const SCREEN_DIMENSION: (u32, u32) = (50, 6);

#[derive(Debug, Clone, Hash)]
pub struct Screen {
    pixels: Vec<Vec<bool>>,
    width: u32,
    height: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: vec![vec![false; width as usize]; height as usize],
            width,
            height,
        }
    }

    pub fn run(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(width, height) => {
                for y in 0..height {
                    for x in 0..width {
                        self.pixels[y as usize][x as usize] = true;
                    }
                }
            }
            Instruction::RotateRow {
                row: row_idx,
                amount,
            } => {
                let mut row = self.pixels[row_idx].clone();
                row.rotate_right(amount as usize);
                self.pixels[row_idx] = row;
            }
            Instruction::RotateColumn {
                column: column_idx,
                amount,
            } => {
                let amount = amount % self.height;
                let mut column = vec![];
                for y in 0..self.height {
                    column.push(self.pixels[y as usize][column_idx]);
                }
                column.rotate_right(amount as usize);
                for y in 0..self.height {
                    self.pixels[y as usize][column_idx] = column[y as usize];
                }
            }
        }
    }

    pub fn count_lit_pixels(&self) -> usize {
        self.pixels.iter().flatten().filter(|x| **x).count()
    }

    pub fn draw(&self) {
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                if *pixel {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum Instruction {
    Rect(u32, u32),
    RotateRow { row: usize, amount: u32 },
    RotateColumn { column: usize, amount: u32 },
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let instruction = if line.starts_with("rect") {
            let (width, height) = line.strip_prefix("rect ").unwrap().split_once('x').unwrap();
            Instruction::Rect(width.parse().unwrap(), height.parse().unwrap())
        } else if line.starts_with("rotate row") {
            let (row, amount) = line
                .strip_prefix("rotate row y=")
                .unwrap()
                .split_once(" by ")
                .unwrap();
            Instruction::RotateRow {
                row: row.parse().unwrap(),
                amount: amount.parse().unwrap(),
            }
        } else if line.starts_with("rotate column") {
            let (column, amount) = line
                .strip_prefix("rotate column x=")
                .unwrap()
                .split_once(" by ")
                .unwrap();
            Instruction::RotateColumn {
                column: column.parse().unwrap(),
                amount: amount.parse().unwrap(),
            }
        } else {
            unreachable!()
        };

        instructions.push(instruction);
    }

    instructions
}

fn part1(input: &str) -> usize {
    let instructions = parse_input(input);

    let mut screen = Screen::new(SCREEN_DIMENSION.0, SCREEN_DIMENSION.1);

    for instruction in instructions {
        screen.run(instruction);
    }

    screen.draw();
    screen.count_lit_pixels()
}

// Part 2 is just making out the code that's printed. part1 already does that.

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}
