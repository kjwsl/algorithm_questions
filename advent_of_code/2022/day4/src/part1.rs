use std::ops::RangeInclusive;

// struct MyRange {
//     range: RangeInclusive<usize>,
//     has_merged: bool,
// }

fn range_includes_another<I>(range1: RangeInclusive<I>, range2: RangeInclusive<I>) -> bool
where
    I: std::cmp::PartialOrd,
{
    let (start1, end1) = (range1.start(), range1.end());
    let (start2, end2) = (range2.start(), range2.end());

    (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)
}

pub fn solve(input: &str) -> i32 {
    let pairs = input.lines().collect::<Vec<_>>();
    let mut cnt = 0;
    for pair in pairs {
        let (pair1, pair2) = pair.split_once(",").expect("No way!");
        let (start, end) = pair1.split_once("-").expect("wow");
        let range1 = start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap();
        let (start, end) = pair2.split_once("-").expect("wow");
        let range2 = start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap();

        if range_includes_another(range1, range2) {
            cnt += 1;
        }
    }
    cnt
}
