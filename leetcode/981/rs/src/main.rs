use std::{cell::RefCell, collections::HashMap};

fn main() {
    println!("Hello, world!");
}
struct TimeMap {
    map:HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .or_default()
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map.get(&key).and_then(|values| {
            match values.binary_search_by_key(&timestamp, |&(t, _)| t) {
                Ok(idx) => Some(values[idx].1.clone()),
                Err(0) => None,
                Err(idx) => Some(values[idx - 1].1.clone()),
            }
        }).unwrap_or("".to_string())
    }
}
