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

    if grid.width < 2 || grid.height < 2 {
        return grid.width * grid.height;
    }

    let edge_count = grid.width * 2 + grid.height * 2 - 4;
    grid.get_visible_tree_count(1..grid.width - 1, 1..grid.height - 1) + edge_count
}
