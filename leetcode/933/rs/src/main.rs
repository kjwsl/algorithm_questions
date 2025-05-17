use std::cell::RefCell;

struct RecentCounter {
    pings: RefCell<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            pings: RefCell::new(vec![]),
        }
    }

    fn ping(&self, t: i32) -> i32 {
        let mut pings = self.pings.borrow_mut();
        pings.push(t);

        pings
            .iter()
            .filter(|&ping| *ping >= t - 3000 && *ping <= t)
            .count() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

fn main() {}
