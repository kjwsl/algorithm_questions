use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Hash)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: Vec::new(),
        }
    }

    fn add_row(&mut self, ch: &[char]) {
        self.data.extend_from_slice(ch);
    }

    fn get(&self, x: usize, y: usize) -> char {
        let y = y % self.height;
        let x = x % self.width;
        self.data[y * self.width + x]
    }
}

fn parse_input(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();
    let mut map = Map::new(lines[0].len(), lines.len());

    for line in input.lines() {
        map.add_row(line.chars().collect::<Vec<_>>().as_slice());
    }

    map
}

fn count_tree_for_movement(map: &Map, movement: (i32, i32)) -> usize {
    let mut count = 0;

    let mut x = 0usize;
    let mut y = 0usize;

    while y < map.height {
        if map.get(x, y) == '#' {
            count += 1;
        }
        x = (x + movement.0 as usize) % map.width;
        y += movement.1 as usize;
    }

    count
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);

    const MOVEMENT: (i32, i32) = (3, 1);

    count_tree_for_movement(&map, MOVEMENT)
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);

    let movements = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    movements
        .iter()
        .map(|m| count_tree_for_movement(&map, *m))
        .product()
}

fn main() {
    let file_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Using '../input.txt' as default input file");
            "../input.txt".to_string()
        }
    };
    let input = fs::read_to_string(file_path).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
