use std::fs::read_to_string;

use crate::{
    part1,
    utils::{HailStone, TestArea, TestAreaInBounds, Vec3},
};
#[test]
fn test_solve() {
    let txt = read_to_string("test.txt").unwrap();
    let ta = TestArea::new((7, 27), (7, 27));

    let cnt = part1::solve(&txt, &ta);

    assert_eq!(cnt, 2);
}
