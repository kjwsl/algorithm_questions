use rust::Solution;

fn main() {
    let q = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    let ans = Solution::minimum_total(q);
    println!("{:?}", ans);

    let q = vec![vec![-10]];
    let ans = Solution::minimum_total(q);
    println!("{:?}", ans);
}
