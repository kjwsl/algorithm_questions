use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            robots: Vec::new(),
        }
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }

    pub fn add_robots(&mut self, robots: Vec<Robot>) {
        self.robots.extend(robots);
    }

    pub fn move_robots_n_times(&mut self, n: usize) {
        for robot in self.robots.iter_mut() {
            for _ in 0..n {
                robot.pos.0 = (robot.pos.0 as isize + robot.direction.0).rem_euclid(self.width as isize) as usize;
                robot.pos.1 = (robot.pos.1 as isize + robot.direction.1).rem_euclid(self.height as isize) as usize;
            }
        }
    }

    pub fn split_robots_into_quadrants(&self) -> [Vec<Robot>; 4] {
        let half_width = self.width / 2;
        let half_height = self.height / 2;

        let q1 = self
            .robots
            .iter()
            .filter(|robot| robot.pos.0 < half_width && robot.pos.1 < half_height)
            .cloned()
            .collect();

        let q2 = self
            .robots
            .iter()
            .filter(|robot| robot.pos.0 >= self.width - half_width && robot.pos.1 < half_height)
            .cloned()
            .collect();

        let q3 = self
            .robots
            .iter()
            .filter(|robot| robot.pos.0 < half_width && robot.pos.1 >= self.height - half_height)
            .cloned()
            .collect();

        let q4 = self
            .robots
            .iter()
            .filter(|robot| robot.pos.0 >= self.width - half_width && robot.pos.1 >= self.height - half_height)
            .cloned()
            .collect();

        [q1, q2, q3, q4]
    }

    pub fn print(&self) {
        let mut robot_pos = HashMap::new();

        self.robots.iter().for_each(|robot| {
            *robot_pos.entry(robot.pos).or_insert(0) += 1;
        });
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    match robot_pos.get(&(x, y)) {
                        Some(n) => n.to_string(),
                        None => ".".to_string(),
                    }
                );
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    pos: (usize, usize),
    direction: (isize, isize),
}

impl Robot {
    fn new(pos: (usize, usize), direction: (isize, isize)) -> Self {
        Self { pos, direction }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (pos_x, pos_y) = p[2..].split_once(',').unwrap();
            let (pos_x, pos_y) = (
                pos_x.parse::<usize>().unwrap(),
                pos_y.parse::<usize>().unwrap(),
            );

            let (direction_x, direction_y) = v[2..].split_once(',').unwrap();
            let (direction_x, direction_y) = (
                direction_x.parse::<isize>().unwrap(),
                direction_y.parse::<isize>().unwrap(),
            );

            Robot::new((pos_x, pos_y), (direction_x, direction_y))
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let robots = parse_input(input);
    let mut map = Map::new(101, 103);
    map.add_robots(robots);

    map.move_robots_n_times(100);

    let quadrants = map.split_robots_into_quadrants();

    quadrants.iter().map(|q| q.len() as u32).product()
}

fn part2(input: &str) -> usize {
    let robots = parse_input(input);
    let mut map = Map::new(101, 103);
    map.add_robots(robots);

    for i in 0..2000 {
        println!("step {}", i);
        map.move_robots_n_times(1);
        map.print();
    }

    todo!()
}

fn main() {
    let filepath = std::env::args()
        .nth(1)
        .unwrap_or("../input.txt".to_string());

    let input = std::fs::read_to_string(filepath).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
