use crate::utils::*;

pub fn solve(txt: &str, test_area: &TestArea) -> u64 {
    let mut cnt = 0u64;
    let lines = txt.lines().collect::<Vec<&str>>();
    // for every two lines
    // extract the info

    let mut i = 0usize;
    while i < lines.len() - 1 {
        let mut j = i + 1;
        while j < lines.len() {
            let hailstone = extract_info(lines[i]);
            let next_hailstone = extract_info(lines[j]);
            let collision_point = hailstone.get_collision_point(&next_hailstone);
            if let Some(point) = collision_point {
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
