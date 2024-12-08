use std::ops::Range;
struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Grid {
        let width = grid[0].len();
        let height = grid.len();
        Grid {
            grid,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let tree = self.get(x, y);

        if (0..x).all(|i| self.get(i, y) < tree) {
            return true;
        }

        if (x + 1..self.width).all(|i| self.get(i, y) < tree) {
            return true;
        }

        if (0..y).all(|i| self.get(x, i) < tree) {
            return true;
        }

        if (y + 1..self.height).all(|i| self.get(x, i) < tree) {
            return true;
        }

        false
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> usize {
        let tree = self.get(x, y);

        // Count to the left
        let mut score_left = 0;
        for i in (0..x).rev() {
            score_left += 1;
            if self.get(i, y) >= tree {
                break;
            }
        }

        // Count to the right
        let mut score_right = 0;
        for i in x + 1..self.width {
            score_right += 1;
            if self.get(i, y) >= tree {
                break;
            }
        }

        // Count upwards
        let mut score_up = 0;
        for i in (0..y).rev() {
            score_up += 1;
            if self.get(x, i) >= tree {
                break;
            }
        }

        // Count downwards
        let mut score_down = 0;
        for i in y + 1..self.height {
            score_down += 1;
            if self.get(x, i) >= tree {
                break;
            }
        }

        score_left * score_right * score_up * score_down
    }

    fn get_max_scenic_score(&self) -> usize {
        let mut max_score = 0;
        let mut max_pos = (0, 0);
        for y in 0..self.height {
            for x in 0..self.width {
                let score = self.get_scenic_score(x, y);
                if score > max_score {
                    max_score = score;
                    max_pos = (x, y);
                }
            }
        }
        println!("Max score: {} at {:?}", max_score, max_pos);
        max_score
    }

    fn get_visible_tree_count(&self, width_rng: Range<usize>, height_rng: Range<usize>) -> usize {
        let mut count = 0;
        for y in height_rng {
            for x in width_rng.clone() {
                if self.is_tree_visible(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn solve(input: &str) -> usize {
    let grid = Grid::new(
        input
            .lines()
            .map(|c| c.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    grid.get_max_scenic_score()
}
