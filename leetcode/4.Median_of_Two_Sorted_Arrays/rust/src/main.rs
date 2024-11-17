struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = nums1
            .iter()
            .chain(nums2.iter())
            .copied()
            .collect::<Vec<i32>>();

        merged.sort_unstable();

        let size = merged.len() - 1;
        if size % 2 == 0 {
            (merged[size / 2] + merged[size / 2 + 1]) as f64 / 2f64
        } else {
            merged[size / 2] as f64
        }
    }
}
