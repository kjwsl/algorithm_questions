use std::collections::{BinaryHeap, HashMap, LinkedList, VecDeque};

struct LRUCache {
    map: HashMap<i32, i32>,
    list: LinkedList<i32>,
    history: VecDeque<i32>,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            list: LinkedList::new(),
            history: VecDeque::new(),
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(&val) => {
                self.update_history(key);
                val
            }
            None => -1,
        }
    }

    fn update_history(&mut self, key: i32) {
        if let Some(pos) = self.history.iter().position(|&k| k == key) {
            self.history.remove(pos);
        }

        self.history.push_front(key);

        if self.history.len() as i32 > self.capacity {
            if let Some(key) = self.history.pop_back() {
                self.map.remove(&key);
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if !self.map.contains_key(&key) {
            while self.map.len() as i32 >= self.capacity {
                let key = self.history.pop_back().expect("no history");
                self.map.remove(&key);
            }
        }
        self.map.insert(key, value);
        self.update_history(key);
    }
}
