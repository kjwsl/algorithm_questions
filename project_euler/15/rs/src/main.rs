use std::ops::{Range, RangeInclusive};

use num_bigint::BigUint;

trait Product {
    type Target;
    fn product(&self) -> Self::Target;
}

impl Product for Range<BigUint> {
    type Target = BigUint;
    fn product(&self) -> Self::Target {
        let mut res = BigUint::from(1u32);
        let mut current = self.start.clone();

        while current < self.end {
            res *= &current;
            current += 1u32;
        }

        res
    }
}

impl Product for RangeInclusive<BigUint> {
    type Target = BigUint;
    fn product(&self) -> Self::Target {
        let mut res = BigUint::from(1u32);
        let mut current = self.start().clone();

        while current <= *self.end() {
            res *= &current;
            current += 1u32;
        }

        res
    }
}

fn count_routes_in_grid(n: usize) -> BigUint {
    let n = BigUint::from(n);
    let total_pos = n.clone() * BigUint::from(2u32);
    (BigUint::from(1u32)..=total_pos.clone()).product()
        / ((BigUint::from(1u32)..=n.clone()).product()
            * (BigUint::from(1u32)..=total_pos - n.clone()).product())
}

fn main() {
    println!("{}", count_routes_in_grid(20));
}
