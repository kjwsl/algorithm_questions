struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        let mut lidx = 0usize;
        let mut ridx = 0usize;
        for _ in 0..k {
            let left = nums1[lidx];
            let right = nums2[ridx];
            ans.push(vec![left, right]);

            if nums1.len() - lidx > 1 && nums2.len() - ridx > 1 {
                if nums1[lidx + 1] <= nums2[ridx + 1] {
                    lidx += 1;
                    continue;
                } else {
                    lidx = 0;
                    ridx += 1;
                    continue;
                }
            } else if nums1.len() - lidx > 2 {
                lidx += 1;
                continue;
            } else if nums2.len() - ridx > 2 {
                ridx += 1;
                lidx = 0;
                continue;
            }
        }

        ans
    }
}
