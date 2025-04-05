use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn part1(input: &str) -> u32 {
        let map = TopographicMap::new(input);
        map.count_all_trailheads(true)
    }
    pub fn part2(input: &str) -> u32 {
        let map = TopographicMap::new(input);
        map.count_all_trailheads(false)
    }
}

#[derive(Debug, Clone)]
struct TopographicMap {
    map: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl TopographicMap {
    pub fn new(input: &str) -> Self {
        let map: Vec<Vec<i32>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
        let width = map[0].len();
        let height = map.len();
        TopographicMap { map, width, height }
    }

    pub fn count_all_trailheads(&self, unique: bool) -> u32 {
        let grounds = self.find_grounds();
        if grounds.is_empty() {
            return 0;
        }
        let n = grounds.len();
        let mut res = vec![0; n];
        for (i, &ground) in grounds.iter().enumerate() {
            let mut upwards = vec![];
            let mut start_points = vec![ground];
            let mut checked = HashSet::new();
            // We are hiking!
            for expected in 1..=9 {
                checked.clear();
                // We will collect starting points now!
                // It's only gonna get bigger...
                // We can go any of the four directions!
                for direction in Direction::SEQ {
                    for coor in &start_points {
                        if let Some(coor) = self.mv_coor(*coor, direction) {
                            if unique {
                                if checked.contains(&coor) {
                                    continue;
                                } else {
                                    checked.insert(coor);
                                }
                            }
                            if let Some(&height) = self.get(coor) {
                                if height == expected {
                                    upwards.push(coor);
                                }
                            }
                        }
                    }
                }
                start_points = upwards;
                upwards = vec![];
            }

            res[i] += start_points.len() as u32;
        }

        res.iter().sum()
    }

    pub fn get(&self, coor: Coordinates) -> Option<&i32> {
        self.map.get(coor.y)?.get(coor.x)
    }

    fn find_grounds(&self) -> Vec<Coordinates> {
        let mut paths = vec![];
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, height) in row.iter().enumerate() {
                if *height == 0 {
                    paths.push(Coordinates::new(col_idx, row_idx));
                }
            }
        }
        paths
    }

    fn mv_coor(&self, coor: Coordinates, direction: Direction) -> Option<Coordinates> {
        let x: usize;
        let y: usize;
        match direction {
            Direction::Left => {
                x = coor.x.checked_sub(1)?;
                y = coor.y;
            }
            Direction::Down => {
                x = coor.x;
                y = coor.y.checked_add(1)?;

                if y >= self.height {
                    return None;
                }
            }
            Direction::Up => {
                x = coor.x;
                y = coor.y.checked_sub(1)?;
            }
            Direction::Right => {
                x = coor.x.checked_add(1)?;
                y = coor.y;

                if x >= self.width {
                    return None;
                }
            }
        }
        Some(Coordinates { x, y })
    }
}

#[derive(Debug, Clone)]
struct Trailhead {
    path: VecDeque<Coordinates>,
}

impl Trailhead {
    pub fn new(path: VecDeque<Coordinates>) -> Self {
        Self { path }
    }

    pub fn push_front(&mut self, coor: Coordinates) {
        self.path.push_front(coor);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    const SEQ: [Self; 4] = [Self::Left, Self::Up, Self::Right, Self::Down];
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "0123\n1234\n8765\n9876";
    const SAMPLE2: &str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
    #[test]
    fn test_part1_1() {
        assert_eq!(Solution::part1(SAMPLE1), 1);
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(Solution::part1(SAMPLE2), 36);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(Solution::part2(SAMPLE2), 81)
    }
}
