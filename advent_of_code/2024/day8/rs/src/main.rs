use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn all(value: i32) -> Self {
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
        self.frequencies.entry(freq).or_default().insert(point);
    }

    pub fn calculate_antinodes(&mut self) {
        for values in self.frequencies.values() {
            let values: Vec<_> = values.iter().collect();

            for i in 0..values.len() {
                for j in i + 1..values.len() {
                    let antinode1 = *values[i] * Point::all(2) - *values[j];
                    let antinode2 = *values[j] * Point::all(2) - *values[i];

                    if (0..self.width).contains(&(antinode1.x as usize))
                        && (0..self.height).contains(&(antinode1.y as usize))
                    {
                        self.antinodes.insert(antinode1);
                    }

                    if (0..self.width).contains(&(antinode2.x as usize))
                        && (0..self.height).contains(&(antinode2.y as usize))
                    {
                        self.antinodes.insert(antinode2);
                    }
                }
            }
        }
    }

    pub fn calculate_antinodes2(&mut self) {
        for values in self.frequencies.values() {
            let values: Vec<_> = values.iter().collect();

            for i in 0..values.len() {
                for j in i + 1..values.len() {
                    let slope = *values[j] - *values[i];
                    (-100..100).for_each(|offset| {
                        let antinode = *values[i] + slope * Point::all(offset);
                        if (0..self.width).contains(&(antinode.x as usize))
                            && (0..self.height).contains(&(antinode.y as usize))
                        {
                            self.antinodes.insert(antinode);
                        }
                    })
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
                map.add_frequency(c, Point { x: j as i32, y: i as i32 });
            }
        }
    }

    map
}

fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    map.calculate_antinodes();

    map.antinodes.len()
}

fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    map.calculate_antinodes2();

    map.antinodes.len()
}

fn main() {
    let filepath = match std::env::args().nth(1) {
        Some(filepath) => filepath,
        None => "../input.txt".to_string(),
    };

    let input = std::fs::read_to_string(filepath).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
