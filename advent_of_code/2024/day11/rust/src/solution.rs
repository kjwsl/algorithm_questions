use std::{collections::HashMap, sync::mpsc::channel};

use threadpool::ThreadPool;

type BlinkMemo = HashMap<i64, Vec<i64>>;

pub struct Solution {
    pub nums: Vec<i64>,
    pub memo: BlinkMemo,
}

impl Solution {
    fn num_digits(num: i64) -> i32 {
        let mut num = num;
        let mut count = 0;
        while num != 0 {
            num /= 10;
            count += 1;
        }

        count
    }

    fn split_number(num: i64) -> (i64, i64) {
        let num_str = num.to_string();
        let (num1, num2) = num_str.split_at(num_str.len() / 2);
        let num1 = num1.parse::<i64>().unwrap();
        let num2 = num2.parse::<i64>().unwrap();
        (num1, num2)
    }

    fn blink(num: i64, memo: &mut BlinkMemo) -> Vec<i64> {
        if let Some(nums) = memo.get(&num) {
            return nums.clone();
        }

        let mut new_nums = Vec::with_capacity(2);
        if num == 0 {
            new_nums.push(1);
        } else if num >= 10 && Self::num_digits(num) % 2 == 0 {
            let (num1, num2) = Self::split_number(num);
            new_nums.push(num1);
            new_nums.push(num2);
        } else {
            new_nums.push(num * 2024);
        }

        memo.insert(num, new_nums.clone());

        new_nums
    }
    pub fn new(input: &str) -> Self {
        Self {
            nums: input
                .split(" ")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect(),
            memo: HashMap::new(),
        }
    }

    pub fn solve_part1(&mut self) -> i32 {
        let pool = ThreadPool::new(8);
        let (tx, rx) = channel();
        let mut nums: Vec<i64> = Vec::new();
        for _ in 0..25 {
            let tx = tx.clone();
            pool.execute(move || {
                let mut nums = Vec::new();
                for num in self.nums.iter() {
                    nums.extend(Self::blink(*num, &mut self.memo));
                }
                tx.send(nums.len() as i32).unwrap();
            });
        }
        let count = rx.iter().take(25).sum();
        count
    }

    pub fn solve_part2(&mut self) -> i32 {
        for _ in 0..75 {
            self.nums = self.blink();
        }

        self.nums.len() as i32
    }
}
