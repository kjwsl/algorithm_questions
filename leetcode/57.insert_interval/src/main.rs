fn solve(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut new_interval = new_interval;
    let mut i = 0;
    while i < intervals.len() && intervals[i][1] < new_interval[0] {
        res.push(intervals[i].clone());
        i += 1;
    }
    while i < intervals.len() && intervals[i][0] <= new_interval[1] {
        new_interval[0] = new_interval[0].min(intervals[i][0]);
        new_interval[1] = new_interval[1].max(intervals[i][1]);
        i += 1;
    }
    res.push(new_interval);
    while i < intervals.len() {
        res.push(intervals[i].clone());
        i += 1;
    }
    res
}

fn main() {
    test(
        vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ],
        vec![4, 8],
        vec![vec![1, 2], vec![3, 10], vec![12, 16]],
    );
    test(
        vec![vec![1, 3], vec![6, 9]],
        vec![2, 5],
        vec![vec![1, 5], vec![6, 9]],
    );
}

fn test(interval: Vec<Vec<i32>>, new_interval: Vec<i32>, expected: Vec<Vec<i32>>) {
    unsafe {
        TEST_NUM += 1;
    }
    static mut TEST_NUM: u32 = 0;
    let res = solve(interval, new_interval);
    let success = res == expected;
    println!(
        "Test #{}: {}",
        unsafe { TEST_NUM },
        if success { "PASSED" } else { "FAILED" }
    );
    if !success {
        println!("Expected: {:?}", expected);
        println!("Got: {:?}", res);
    }
}
