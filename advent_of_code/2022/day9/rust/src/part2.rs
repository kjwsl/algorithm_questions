use std::collections::HashSet;

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    routes: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            routes: HashSet::new(),
        }
    }

    fn tail_need_move(&self) -> bool {
        let (head_x, head_y) = self.head;
        let (tail_x, tail_y) = self.tail;

        if (tail_x - head_x).abs() > 1 || (tail_y - head_y).abs() > 1 {
            return true;
        }

        false
    }

    fn mv(&mut self, direction: char) {
        let (x, y) = self.head;

        match direction {
            'U' => {
                self.head = (x, y + 1);
            }
            'D' => {
                self.head = (x, y - 1);
            }
            'L' => {
                self.head = (x - 1, y);
            }
            'R' => {
                self.head = (x + 1, y);
            }
            _ => panic!("Invalid direction"),
        }

        if self.tail_need_move() {
            match direction {
                'U' => {
                    self.tail = (self.head.0, self.head.1 - 1);
                }
                'D' => {
                    self.tail = (self.head.0, self.head.1 + 1);
                }
                'L' => {
                    self.tail = (self.head.0 + 1, self.head.1);
                }
                'R' => {
                    self.tail = (self.head.0 - 1, self.head.1);
                }
                _ => panic!("Invalid direction"),
            }
        }

        self.routes.insert(self.head);
    }

    fn mv_times(&mut self, direction: char, times: i32) {
        for _ in 0..times {
            self.mv(direction);
        }
    }

    fn distance(&self) -> i32 {
        self.head.0.abs() + self.head.1.abs()
    }
}

pub fn solve(input: &str) -> usize {
    let instructions: Vec<&str> = input.lines().collect();

    let mut rope = Rope::new();

    for inst in instructions {
        let (direction, times) = inst
            .split_once(" ")
            .map(|(a, b)| (a.chars().next().unwrap(), b.parse::<i32>().unwrap()))
            .unwrap();

        rope.mv_times(direction, times);
    }

    rope.routes.len()
}
