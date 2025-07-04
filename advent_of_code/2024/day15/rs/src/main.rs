#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
enum Object {
    #[default]
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct State {
    pub map: Map,
    pub moves: Vec<Move>,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            moves: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    pub map: Vec<Vec<Object>>,
    pub robot: Option<(usize, usize)>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            robot: None,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Object> {
        self.map.get(y)?.get(x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Object> {
        self.map.get_mut(y)?.get_mut(x)
    }

    pub fn width(&self) -> usize {
        match self.map.first() {
            Some(row) => row.len(),
            None => 0,
        }
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }
}

struct Solution {
    input: &'static str,
}

impl Solution {
    pub fn new(input: &'static str) -> Self {
        Self { input }
    }

    pub fn part1(&self) -> i32 {
        let mut state = Self::parse_input(self.input);

        for move_ in state.moves.clone() {
            let robot_pos = state.map.robot.unwrap();
            dbg!(move_);
            dbg!(robot_pos);
            if Solution::move_object(&mut state, robot_pos, move_) {
                println!("Moved from {:?} to {:?}", robot_pos, state.map.robot);
            } else {
                println!("Robot stays still at {:?}", robot_pos);
                if state.map.get(robot_pos.0, robot_pos.1) != Some(&Object::Robot) {
                    panic!("Robot moved to invalid position");
                }
            }
        }

        Self::sum_box_gps_coords(&state)
    }

    fn sum_box_gps_coords(state: &State) -> i32 {
        let mut sum = 0;
        for row in 0..state.map.height() {
            for col in 0..state.map.width() {
                if state.map.get(col, row) == Some(&Object::Box) {
                    sum += row as i32 * 100 + col as i32;
                }
            }
        }

        sum
    }

    fn move_object(state: &mut State, object_pos: (usize, usize), move_: Move) -> bool {
        let new_pos = match move_ {
            Move::Left => (object_pos.0.saturating_sub(1), object_pos.1),
            Move::Right => ((object_pos.0 + 1).clamp(0, state.map.width()), object_pos.1),
            Move::Up => (object_pos.0, object_pos.1.saturating_sub(1)),
            Move::Down => (
                object_pos.0,
                (object_pos.1 + 1).clamp(0, state.map.height()),
            ),
        };

        let object = *state.map.get(object_pos.0, object_pos.1).unwrap();
        println!("object: {:?}", object);

        if object == Object::Wall || object == Object::Empty {
            return false;
        }

        if object_pos == new_pos {
            return false;
        }

        match state
            .map
            .get(new_pos.0, new_pos.1)
            .expect("Invalid move for robot")
        {
            Object::Empty => {
                println!(
                    "Moving {:?} from {:?} to {:?} (empty)",
                    object, object_pos, new_pos
                );
                *state.map.get_mut(object_pos.0, object_pos.1).unwrap() = Object::Empty;
                *state.map.get_mut(new_pos.0, new_pos.1).unwrap() = object;
                if object == Object::Robot {
                    state.map.robot = Some(new_pos);
                }

                true
            }
            Object::Wall => false,
            Object::Box => {
                if Self::move_object(state, new_pos, move_) {
                    *state.map.get_mut(object_pos.0, object_pos.1).unwrap() = Object::Empty;
                    *state.map.get_mut(new_pos.0, new_pos.1).unwrap() = object;
                    if object == Object::Robot {
                        state.map.robot = Some(new_pos);
                    }
                    true
                } else {
                    false
                }
            }
            Object::Robot => {
                panic!("Invalid move for robot")
            }
        }
    }

    fn parse_input(input: &str) -> State {
        let mut state = State::new();

        let mut map = Map::new();
        let mut moves = Vec::new();
        let mut lines = input.lines();

        for (i, line) in lines.by_ref().enumerate() {
            if line.is_empty() {
                break;
            }
            let mut row = Vec::new();

            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Object::Empty),
                    '#' => row.push(Object::Wall),
                    'O' => row.push(Object::Box),
                    '@' => {
                        row.push(Object::Robot);
                        map.robot = Some((j, i))
                    }
                    _ => unreachable!(),
                }
            }

            map.map.push(row);
        }

        for line in lines {
            for c in line.chars() {
                match c {
                    '<' => moves.push(Move::Left),
                    '>' => moves.push(Move::Right),
                    '^' => moves.push(Move::Up),
                    'v' => moves.push(Move::Down),
                    _ => unreachable!(),
                }
            }
        }

        state.map = map;
        state.moves = moves;

        state
    }
}

fn main() {
    let solution = Solution::new(include_str!("../../input.txt"));
    println!("Part 1: {}", solution.part1());
}
