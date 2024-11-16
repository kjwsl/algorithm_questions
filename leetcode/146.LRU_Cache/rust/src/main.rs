struct LRUCache {
    capacity: i32,
    cache: std::collections::HashMap<i32, i32>,
    lru: std::collections::VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            cache: std::collections::HashMap::new(),
            lru: std::collections::VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.cache.get(&key) {
            Some(val) => {
                self.lru
                    .remove(self.lru.iter().position(|&x| x == key).unwrap());
                self.lru.push_front(key);
                val.to_owned()
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            self.cache.insert(key, value);
            self.lru
                .remove(self.lru.iter().position(|&x| x == key).unwrap());
            self.lru.push_front(key);
        } else {
            if self.cache.len() == self.capacity as usize {
                let lru_key = self.lru.pop_back().unwrap();
                self.cache.remove(&lru_key);
            }
            self.cache.insert(key, value);
            self.lru.push_front(key);
        }
    }
}
