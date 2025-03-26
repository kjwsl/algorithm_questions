use std::{cell::RefCell, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }

        let mut primes = vec![true; n as usize];
        primes[0] = false;
        primes[1] = false;

        (2..n).for_each(|i| {
            if primes[i as usize] {
                let mut j = i * 2;
                while j < n {
                    primes[j as usize] = false;
                    j += i;
                }
            }
        });

        primes.iter().filter(|&&x| x).count() as i32
    }
}
