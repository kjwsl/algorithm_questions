use crate::utils::TestAreaInBounds;
#[cfg(test)]
use crate::utils::{HailStone, TestArea, Vec3};

#[test]
fn test_get_collision_point() {
    let tolerance = 1e-2;
    let test_area = TestArea::new((7, 27), (7, 27));
    let mut hailstone = HailStone::new(Vec3::new(19, 13, 30), Vec3::new(-2, 1, -2));
    let mut next_hailstone = HailStone::new(Vec3::new(18, 19, 22), Vec3::new(-1, -1, -2));
    let mut expected = Vec3::new(14.333, 15.333, 1.0);

    match hailstone.get_collision_point(&next_hailstone) {
        Some(point) => {
            assert!(&point.is_in_bounds(&test_area));
            assert!((&point.x - &expected.x).abs() < tolerance);
            assert!((&point.y - &expected.y).abs() < tolerance);
        }
        None => panic!("Expected a collision point"),
    }

    hailstone = HailStone::new(Vec3::new(19, 13, 30), Vec3::new(-2, 1, -2));
    next_hailstone = HailStone::new(Vec3::new(20, 25, 34), Vec3::new(-2, -2, -4));
    expected = Vec3::new(11.667, 16.667, 0.0);

    match hailstone.get_collision_point(&next_hailstone) {
        Some(point) => {
            assert!(point.is_in_bounds(&test_area));
            assert!((&point.x - &expected.x).abs() < tolerance);
            assert!((&point.y - &expected.y).abs() < tolerance);
        }
        None => panic!("Expected a collision point"),
    }

    hailstone = HailStone::new(Vec3::new(19, 13, 30), Vec3::new(-2, 1, -2));
    next_hailstone = HailStone::new(Vec3::new(12, 31, 28), Vec3::new(-1, -2, -1));
    expected = Vec3::new(6.2, 19.4, 0.0);
    match hailstone.get_collision_point(&next_hailstone) {
        Some(point) => {
            assert!(!point.is_in_bounds(&test_area));
            assert!((&point.x - &expected.x).abs() < tolerance);
            assert!((&point.y - &expected.y).abs() < tolerance);
        }
        None => panic!("Expected a collision point"),
    }
}
