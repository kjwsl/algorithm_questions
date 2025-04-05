use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet, hash_map::Entry},
};

pub struct Solution;

impl Solution {
    pub fn part1(input: &str) -> i64 {
        let garden = Garden::from_input(input);
        let region_map = garden.compute_region_map();
        println!(
            "region map: {:?}",
            region_map
                .values()
                .map(|r| r
                    .iter()
                    .map(|p| (
                        p.plant_type,
                        p.area,
                        p.perimeter,
                        p.plants
                            .clone()
                            .iter()
                            .map(|p| (p.x, p.y))
                            .collect::<Vec<_>>()
                    ))
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );

        region_map
            .values()
            .map(|regions| regions.iter().map(|r| r.compute_fence_price()).sum::<i64>())
            .sum()
    }

    pub fn part2(input: &str) -> i64 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Hash, Default)]
struct Boundary {
    left: bool,
    up: bool,
    right: bool,
    down: bool,
}

impl Boundary {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone, Default, Hash)]
struct Plant {
    plant_type: char,
    x: usize,
    y: usize,
    boundary: Boundary,
}

#[derive(Debug, Clone, Default)]
struct Region {
    plant_type: char,
    plants: Vec<Plant>,
    area: usize,
    perimeter: usize,
}

impl Region {
    pub fn new(plant_type: char) -> Self {
        Self {
            plant_type,
            ..Default::default()
        }
    }

    pub fn push(&mut self, plant: Plant) -> bool {
        if self.plant_type == plant.plant_type {
            self.plants.push(plant);
            self.update_region();
            true
        } else {
            false
        }
    }

    pub fn compute_fence_price(&self) -> i64 {
        self.area
            .checked_mul(self.perimeter)
            .expect("Failed to calculate fence price") as i64
    }

    pub fn extend(&mut self, other: Region) {
        self.plants.extend(other.plants);
        self.update_region();
    }

    pub fn region_adj(&self, region: &Region) -> bool {
        region.plants.iter().any(|p| self.plant_adj(p))
    }

    pub fn plant_adj(&self, plant: &Plant) -> bool {
        let pos_map: HashSet<(usize, usize)> =
            HashSet::from_iter(self.plants.iter().map(|p| (p.x, p.y)));

        let p = plant;
        let mut adjacent_plants = vec![(p.x + 1, p.y), (p.x, p.y + 1)];
        if let Some(y) = p.y.checked_sub(1) {
            adjacent_plants.push((p.x, y));
        }
        if let Some(x) = p.x.checked_sub(1) {
            adjacent_plants.push((x, p.y));
        }

        for adj_p in &adjacent_plants {
            if pos_map.contains(adj_p) {
                return true;
            }
        }

        false
    }

    fn update_region(&mut self) {
        self.area = self.plants.len();
        self.perimeter = self.calculate_perimter();
    }

    fn calculate_perimter(&self) -> usize {
        self.plants.len() * 4 - self.count_overlapping_sides()
    }

    fn calculate_perimter2(&self) -> usize {}

    fn count_overlapping_sides(&self) -> usize {
        let pos_map: HashSet<(usize, usize)> =
            HashSet::from_iter(self.plants.iter().map(|p| (p.x, p.y)));
        let mut cnt = 0;

        for p in &self.plants {
            let mut adjacent_plants = vec![(p.x + 1, p.y), (p.x, p.y + 1)];
            if let Some(y) = p.y.checked_sub(1) {
                adjacent_plants.push((p.x, y));
            }
            if let Some(x) = p.x.checked_sub(1) {
                adjacent_plants.push((x, p.y));
            }

            for adj_p in &adjacent_plants {
                if pos_map.contains(adj_p) {
                    cnt += 1;
                }
            }
        }

        cnt
    }
}

type RegionMap = HashMap<char, Vec<Region>>;

struct Garden {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Garden {
    pub fn from_input(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn compute_region_map(&self) -> RegionMap {
        let mut map = RegionMap::new();

        for row_idx in 0..self.height {
            for col_idx in 0..self.width {
                let plant = self.get_plant(col_idx, row_idx).unwrap();
                match map.entry(plant.plant_type) {
                    Entry::Occupied(mut e) => {
                        let regions = e.get_mut();
                        if let Some(r) = regions.iter_mut().find(|r| r.plant_adj(&plant)) {
                            r.push(plant);
                            continue;
                        }
                        let mut r = Region::new(plant.plant_type);
                        r.push(plant);
                        regions.push(r);
                    }
                    Entry::Vacant(e) => {
                        let mut r = Region::new(plant.plant_type);
                        r.push(plant);
                        e.insert(vec![r]);
                    }
                }
            }
        }

        Self::merge_regions(&mut map);

        map
    }

    fn merge_regions(map: &mut RegionMap) {
        for _ in 0..map.len() {
            let values = map.values_mut();
            for regions in values {
                let mut i = 0;
                while i < regions.len() {
                    let mut j = i + 1;
                    while j < regions.len() {
                        if regions[i].region_adj(&regions[j]) {
                            let new_region = regions.remove(j);
                            regions[i].extend(new_region);
                        }
                        j += 1
                    }
                    i += 1
                }
            }
        }
    }

    pub fn get_plant(&self, x: usize, y: usize) -> Option<Plant> {
        let plant_type = *self.grid.get(y)?.get(x)?;
        Some(Plant {
            plant_type,
            x,
            y,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

    #[test]
    fn test_sample1() {
        let res = Solution::part1(SAMPLE1);
        assert_eq!(res, 1930);
    }
}
