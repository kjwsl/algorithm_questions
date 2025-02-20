fn blink(nums: &Vec<i64>) -> Vec<i64> {
    let mut new_nums = Vec::new();
    for &num in nums {
        if num == 0 {
            new_nums.push(1);
        } else if num.to_string().len() % 2 == 0 {
            let num_str = num.to_string();
            let (num1, num2) = num_str.split_at(num_str.len() / 2);
            let num1 = num1.parse::<i64>().unwrap();
            let num2 = num2.parse::<i64>().unwrap();
            new_nums.push(num1);
            new_nums.push(num2);
        } else {
            new_nums.push(num * 2024);
        }
    }

    new_nums
}
pub fn solve(input: &str) -> i32 {
    let mut nums: Vec<_> = input
        .split(" ")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    for _ in 0..25 {
        nums = blink(&nums);
        println!("{:?}", nums);
    }

    println!("{:?}", nums);
    nums.len() as i32
}
