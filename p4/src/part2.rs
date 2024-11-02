use crate::utils::{get_hailstones, CollisionPoint, HailStone, TestArea};

pub fn solve(txt: &str, test_area: &TestArea) -> u64 {
    let mut sum = 0u64;
    let hailstones = get_hailstones(txt);
    let collision_points: Vec<CollisionPoint> = Vec::new();

    let mut i = 0;
    while i < hailstones.len() {
        let mut j = i + 1;
        while j < hailstones.len() {
            let hailstone1 = &hailstones[i];
            let hailstone2 = &hailstones[j];
            match hailstone1.get_collision_point(hailstone2) {
                Some(collision_point) => {
                    todo!()
                }
                None => {}
            }
            j += 1;
        }
        i += 1;
    }

    dbg!(hailstones);

    sum
}
