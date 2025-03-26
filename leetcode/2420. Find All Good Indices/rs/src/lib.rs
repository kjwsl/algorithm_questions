pub struct Solution;

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res = vec![];
        let mut asc = vec![];
        let mut desc = vec![];

        nums.windows(2).enumerate().for_each(|(idx, w)| {
            if w[0] < w[1] {
                asc.push(idx);
            } else {
                desc.push(idx);
            }
        });

        (k..nums.len() - k).for_each(|i| {
            if asc.iter().filter(|&&x| x >= i - k).any(|&x| x < i) && desc.iter().filter(|&&x| x < i + k).any(|&x| x >= i) {
                return;
            }
            res.push(i as i32);
        });

        println!("asc: {:?}", asc);
        println!("desc: {:?}", desc);

        res
    }
}
