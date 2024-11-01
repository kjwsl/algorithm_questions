use crate::utils::*;

#[test]
fn test_get_collision_point() {
    let hailstone = HailStone::new(Vec3::new(19, 13, 30), Vec3::new(-2, 1, -2));
    let next_hailstone = HailStone::new(Vec3::new(18, 19, 22), Vec3::new(-1, -1, -2));
    let collision_point = hailstone.get_collision_point(&next_hailstone);
    let tolerance = 1e-2;
    let expected = Vec3::new(14.333, 15.333, 1.0);
    println!("{:?}", &collision_point);
    assert!((collision_point.unwrap().x as f64 - expected.x).abs() < tolerance);
}
