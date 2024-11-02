use crate::utils::{HailStone, TestArea, TestAreaInBounds, Vec3};
#[cfg(test)]
#[test]
fn test_get_collision_point() {
    let tolerance = 1e-2;
    let test_area = TestArea::new((7, 27), (7, 27));
    let mut hailstone = HailStone::new(Vec3::new(19.0, 13.0, 30.0), Vec3::new(-2.0, 1.0, -2.0));
    let mut next_hailstone =
        HailStone::new(Vec3::new(18.0, 19.0, 22.0), Vec3::new(-1.0, -1.0, -2.0));
    let mut expected = Vec3::new(14.333, 15.333, 1.0);

    match hailstone.get_collision_point(&next_hailstone) {
        Some(point) => {
            println!("{:?}", point);
            assert!(&point.is_in_bounds(&test_area));
            assert!((&point.x - &expected.x).abs() < tolerance);
            assert!((&point.y - &expected.y).abs() < tolerance);
        }
        None => panic!("Expected a collision point"),
    }

    hailstone = HailStone::new(Vec3::new(19.0, 13.0, 30.0), Vec3::new(-2.0, 1.0, -2.0));
    next_hailstone = HailStone::new(Vec3::new(20.0, 25.0, 34.0), Vec3::new(-2.0, -2.0, -4.0));
    expected = Vec3::new(11.667, 16.667, 0.0);

    match hailstone.get_collision_point(&next_hailstone) {
        Some(point) => {
            assert!(point.is_in_bounds(&test_area));
            assert!((&point.x - &expected.x).abs() < tolerance);
            assert!((&point.y - &expected.y).abs() < tolerance);
        }
        None => panic!("Expected a collision point"),
    }

    hailstone = HailStone::new(Vec3::new(19.0, 13.0, 30.0), Vec3::new(-2.0, 1.0, -2.0));
    next_hailstone = HailStone::new(Vec3::new(12.0, 31.0, 28.0), Vec3::new(-1.0, -2.0, -1.0));
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
