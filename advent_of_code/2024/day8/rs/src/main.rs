use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn all(value: usize) -> Self {
        Self { x: value, y: value }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    pub width: usize,
    pub height: usize,
    pub frequencies: HashMap<char, HashSet<Point>>,
    pub antinodes: HashSet<Point>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            frequencies: HashMap::new(),
            antinodes: HashSet::new(),
        }
    }

    pub fn add_frequency(&mut self, freq: char, point: Point) {
        self.frequencies
            .entry(freq)
            .or_insert(HashSet::new())
            .insert(point);
    }

    pub fn calculate_antinodes(&mut self) {
        for freq in self.frequencies.keys() {
            let values = self.frequencies.get(freq).unwrap();

            for i in 0..values.len() - 1 {
                for j in 1..values.len() {
                    let point1 = values.iter().nth(i).unwrap();
                    let point2 = values.iter().nth(j).unwrap();

                    let antinode1 = *point1 * Point::all(2) - *point2;
                    let antinode2 = *point2 * Point::all(2) - *point1;

                    if (0..self.width).contains(&antinode1.x)
                        && (0..self.height).contains(&antinode1.y)
                    {
                        self.antinodes.insert(antinode1);
                    }
                    if (0..self.width).contains(&antinode2.x)
                        && (0..self.height).contains(&antinode2.y)
                    {
                        self.antinodes.insert(antinode2);
                    }
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();

    let mut map = Map::new(lines[0].len(), lines.len());

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                map.add_frequency(c, Point { x: j, y: i });
            }
        }
    }

    map.calculate_antinodes();

    map
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    println!("Map dimensions: ({}, {})", map.width, map.height);

    map.antinodes.len()
}

fn main() {
    let filepath = match std::env::args().nth(1) {
        Some(filepath) => filepath,
        None => "../input.txt".to_string(),
    };

    let input = std::fs::read_to_string(filepath).unwrap();

    println!("Part 1: {}", part1(&input));
}
