use std::collections::HashSet;

struct Petrol {
    position: (usize, usize),
    direction: (i32, i32),
}

impl Petrol {
    fn rot_clk_90(&mut self) {
        static ROT_SEQ: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        self.direction = *ROT_SEQ
            .iter()
            .cycle()
            .skip_while(|&&a| a != self.direction)
            .nth(1)
            .expect("No more sequence");
    }
}

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
    petrol: Petrol,
}

impl Grid {
    fn new(data: &[Vec<char>]) -> Self {
        let width = data[0].len();
        let height = data.len();
        let data: Vec<char> = data.iter().flat_map(|row| row.iter()).copied().collect();
        let find_petrol = |data: &[char], width: usize| -> Petrol {
            const DIRECTIONS: [(char, (i32, i32)); 4] =
                [('<', (-1, 0)), ('^', (0, -1)), ('v', (0, 1)), ('>', (1, 0))];

            for (y, row) in data.chunks(width).enumerate() {
                for (x, &cell) in row.iter().enumerate() {
                    if let Some(direction) = DIRECTIONS.iter().find(|&(a, _)| cell == *a) {
                        return Petrol {
                            position: (x, y),
                            direction: direction.1,
                        };
                    }
                }
            }
            panic!("Petrol not found");
        };
        let petrol = find_petrol(&data, width);
        Self {
            data,
            width,
            height,
            petrol,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.data[y * self.width + x]
    }

    fn move_petrol(&mut self) -> Option<(usize, usize)> {
        let new_x = self.petrol.position.0 as i32 + self.petrol.direction.0;
        let new_y = self.petrol.position.1 as i32 + self.petrol.direction.1;

        if !(0..self.width as i32).contains(&new_x) || !(0..self.height as i32).contains(&new_y) {
            return None;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if self.get(new_x, new_y) == '#' {
            self.petrol.rot_clk_90();
            return self.move_petrol();
        } else {
            self.petrol.position = (new_x, new_y);
        }

        Some(self.petrol.position)
    }
}

pub fn solve(input: &str) -> i32 {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut grid = Grid::new(&data);

    let mut hash = HashSet::new();

    hash.insert(grid.petrol.position);

    while let Some(pos) = grid.move_petrol() {
        hash.insert(pos);
    }

    hash.len() as i32
}
