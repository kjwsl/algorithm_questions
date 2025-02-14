use rust::Solution;



fn main() {
    let meetings = vec![
        vec![1, 3],
        vec![2, 4],
        vec![3, 6],
        vec![5, 7],
        vec![7, 8],
    ];

    let ans = Solution::most_booked(2, meetings);
    println!("Ans: {}", ans);

}
