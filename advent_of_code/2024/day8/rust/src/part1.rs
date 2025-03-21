use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);
type Map = HashMap<char, Vec<Coord>>;

/// Collect antennas keyed by their frequency (char).
fn find_antennas(rows: &[&str]) -> Map {
    let mut freq_map = Map::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            // Puzzle states antennas can be letters (uppercase/lowercase) or digits;
            // is_ascii_alphanumeric() covers A-Z, a-z, 0-9.
            if c.is_ascii_alphanumeric() {
                freq_map.entry(c).or_default().push((x, y));
            }
        }
    }
    freq_map
}

/// Return a set of unique antinode coordinates for the entire grid.
fn find_antinodes(freq_map: &Map, width: usize, height: usize) -> HashSet<Coord> {
    let mut antinodes = HashSet::new();

    for positions in freq_map.values() {
        // We need at least two antennas of the same frequency to form antinodes
        if positions.len() < 2 {
            continue;
        }

        // For each pair (p1, p2), compute the two reflection points:
        //   P_a = 2*p1 - p2
        //   P_b = 2*p2 - p1
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                // Convert to isize for arithmetic
                let (x1i, y1i) = (x1 as isize, y1 as isize);
                let (x2i, y2i) = (x2 as isize, y2 as isize);

                // Reflection formulas
                let p_a = (2 * x1i - x2i, 2 * y1i - y2i);
                let p_b = (2 * x2i - x1i, 2 * y2i - y1i);

                if in_bounds(p_a, width, height) {
                    antinodes.insert((p_a.0 as usize, p_a.1 as usize));
                }
                if in_bounds(p_b, width, height) {
                    antinodes.insert((p_b.0 as usize, p_b.1 as usize));
                }
            }
        }
    }
    antinodes
}

/// Check if (x, y) is within the puzzle grid.
fn in_bounds((x, y): (isize, isize), width: usize, height: usize) -> bool {
    x >= 0 && x < width as isize && y >= 0 && y < height as isize
}

/// Main solver: returns the number of unique antinode positions.
pub fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    if height == 0 {
        return 0;
    }
    let width = lines[0].len();

    let freq_map = find_antennas(&lines);
    let antinode_set = find_antinodes(&freq_map, width, height);

    // The puzzle asks for how many unique locations contain an antinode
    antinode_set.len()
}

