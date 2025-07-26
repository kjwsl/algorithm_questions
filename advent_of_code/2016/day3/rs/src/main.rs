#[derive(Clone, Debug)]
struct Triangle {
    sides: [u32; 3],
}

impl Triangle {
    pub fn new(side_a: u32, side_b: u32, side_c: u32) -> Self {
        Self {
            sides: [side_a, side_b, side_c],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.sides[0] + self.sides[1] > self.sides[2]
            && self.sides[0] + self.sides[2] > self.sides[1]
            && self.sides[1] + self.sides[2] > self.sides[0]
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut u32> {
        self.sides.get_mut(index)
    }

    pub fn get(&self, index: usize) -> Option<&u32> {
        self.sides.get(index)
    }
}

fn parse_input(input: &str) -> Vec<Triangle> {
    let mut triangles = vec![];
    let lines = input.lines();

    for line in lines {
        let sides: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        triangles.push(Triangle::new(sides[0], sides[1], sides[2]));
    }
    triangles
}

fn parse_input2(input: &str) -> Vec<Triangle> {
    let triangles = parse_input(input);

    let mut new_triangles = vec![];

    for chunk in triangles.chunks_exact(3) {
        let (a, b, c) = (&chunk[0], &chunk[1], &chunk[2]);

        new_triangles.push(Triangle::new(a.sides[0], b.sides[0], c.sides[0]));
        new_triangles.push(Triangle::new(a.sides[1], b.sides[1], c.sides[1]));
        new_triangles.push(Triangle::new(a.sides[2], b.sides[2], c.sides[2]));
    }

    new_triangles
}

fn part1(input: &str) -> u32 {
    let triangles = parse_input(input);
    triangles.iter().filter(|t| t.is_valid()).count() as u32
}
fn part2(input: &str) -> u32 {
    let triangles = parse_input2(input);
    triangles.iter().filter(|t| t.is_valid()).count() as u32
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();

    let ans1 = part1(&input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(&input);
    println!("Part 2: {}", ans2);
}
