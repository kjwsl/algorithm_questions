use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[repr(u8)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    #[inline(always)]
    fn next_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Up => (pos.0, pos.1 - 1),
        }
    }

    #[inline(always)]
    fn rotate_clockwise(&self) -> Direction {
        // Using the fact that we defined the directions in clockwise order
        unsafe { std::mem::transmute((*self as u8 + 1) & 3) }
    }
}

#[derive(Clone)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {
    #[inline]
    fn rotate_clockwise(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

#[inline(always)]
fn simulate_guard(grid: &[u64], start_pos: (i32, i32), obstacle_pos: (i32, i32), 
                 width: i32, height: i32, visited: &mut HashSet<((i32, i32), Direction)>) -> bool {
    visited.clear();
    let mut pos = start_pos;
    let mut dir = Direction::Up;
    visited.insert((pos, dir));

    loop {
        let next_pos = dir.next_pos(pos);

        if next_pos.0 < 0 || next_pos.0 >= width || 
           next_pos.1 < 0 || next_pos.1 >= height {
            return false;
        }

        let idx = (next_pos.1 * width + next_pos.0) as usize;
        if next_pos == obstacle_pos || (grid[idx >> 6] & (1 << (idx & 63))) != 0 {
            dir = dir.rotate_clockwise();
        } else {
            pos = next_pos;
        }

        if !visited.insert((pos, dir)) {
            return visited.len() > 1;
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let total_bits = (width * height) as usize;
    let mut grid = vec![0u64; (total_bits + 63) >> 6];
    let mut start_pos = (0, 0);

    // Convert grid to bitset
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let idx = y * width as usize + x;
            match c {
                '#' => grid[idx >> 6] |= 1 << (idx & 63),
                '^' => start_pos = (x as i32, y as i32),
                _ => {}
            }
        }
    }

    let mut count = 0;
    let mut visited = HashSet::with_capacity((width * height) as usize);

    // Try each empty position
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            if (grid[idx >> 6] & (1 << (idx & 63))) == 0 && (x, y) != start_pos {
                if simulate_guard(&grid, start_pos, (x, y), width, height, &mut visited) {
                    count += 1;
                }
            }
        }
    }

    count
}
