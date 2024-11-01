use crate::utils::*;

pub fn solve(txt: &str, test_area: &TestArea) -> u64 {
    let mut cnt = 0u64;
    let lines = txt.lines().collect::<Vec<&str>>();
    // for every two lines
    // extract the info

    let hailstones = lines
        .iter()
        .map(|x| extract_info(x))
        .collect::<Vec<HailStone>>();

    let mut i = 0usize;
    while i < lines.len() {
        let mut j = i + 1;

        while j < lines.len() {
            let hailstone = &hailstones[i];
            let next_hailstone = &hailstones[j];
            if let Some(point) = hailstone.get_collision_point(next_hailstone) {
                if point.is_in_bounds(&test_area) {
                    cnt += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }

    cnt
}
