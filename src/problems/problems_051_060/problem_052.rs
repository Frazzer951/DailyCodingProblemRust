/* HARD
Implement an LRU (Least Recently Used) cache. It should be able to be
initialized with a cache size n, and contain the following methods:

 * set(key, value): sets key to value. If there are already n items in the cache
   and we are adding a new item, then it should also remove the least recently
   used item.
 * get(key): gets the value at key. If no such key exists, return null.

Each operation should run in O(1) time.
*/

use std::collections::{HashMap, VecDeque};

struct LruCache {
    size: usize,
    next: usize,
    values: Vec<i32>,
    keys: HashMap<String, usize>,
    used: VecDeque<String>,
}

impl LruCache {
    fn new(size: usize) -> Self {
        Self {
            size,
            next: 0,
            values: vec![0; size],
            keys: Default::default(),
            used: Default::default(),
        }
    }

    fn set(&mut self, key: String, value: i32) {
        if self.keys.len() == self.size {
            let oldest = self.used.pop_front().unwrap();
            self.next = self.keys[&oldest];
            self.keys.remove(&oldest);
        }
        self.values[self.next] = value;
        self.keys.insert(key.clone(), self.next);
        self.next += 1;
        self.used.push_back(key);
    }

    fn get(&mut self, key: String) -> Option<i32> {
        if self.keys.contains_key(&key) {
            let index = self.keys[&key];
            let index_element = self.used.iter().position(|x| x == &key).unwrap();
            self.used.remove(index_element);
            self.used.push_back(key);

            Some(self.values[index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_basic_usage() {
        let mut cache = LruCache::new(5);

        cache.set("v1".to_string(), 1);
        assert_eq!(cache.get("v1".to_string()), Some(1));
        cache.set("v2".to_string(), 2);
        assert_eq!(cache.get("v2".to_string()), Some(2));
        cache.set("v3".to_string(), 3);
        assert_eq!(cache.get("v3".to_string()), Some(3));
        cache.set("v1".to_string(), 4);
        assert_eq!(cache.get("v1".to_string()), Some(4));
    }

    #[test]
    fn test_lru_dropped_value() {
        let mut cache = LruCache::new(5);

        cache.set("v1".to_string(), 1);
        cache.set("v2".to_string(), 2);
        cache.set("v3".to_string(), 3);
        cache.set("v4".to_string(), 4);
        cache.set("v5".to_string(), 5);

        assert_eq!(cache.get("v1".to_string()), Some(1));
        assert_eq!(cache.get("v2".to_string()), Some(2));
        assert_eq!(cache.get("v3".to_string()), Some(3));
        assert_eq!(cache.get("v4".to_string()), Some(4));
        assert_eq!(cache.get("v5".to_string()), Some(5));

        cache.set("v6".to_string(), 6);
        assert_eq!(cache.get("v6".to_string()), Some(6));
        assert_eq!(cache.get("v1".to_string()), None);
    }

    #[test]
    fn test_lru_dropped_value_2() {
        let mut cache = LruCache::new(5);

        cache.set("v1".to_string(), 1);
        cache.set("v2".to_string(), 2);
        cache.set("v3".to_string(), 3);
        cache.set("v4".to_string(), 4);
        cache.set("v5".to_string(), 5);

        assert_eq!(cache.get("v1".to_string()), Some(1));
        assert_eq!(cache.get("v2".to_string()), Some(2));
        assert_eq!(cache.get("v3".to_string()), Some(3));
        assert_eq!(cache.get("v4".to_string()), Some(4));
        assert_eq!(cache.get("v5".to_string()), Some(5));

        cache.set("v6".to_string(), 6);
        cache.set("v7".to_string(), 7);
        cache.set("v8".to_string(), 8);
        cache.set("v9".to_string(), 9);
        cache.set("v10".to_string(), 10);

        assert_eq!(cache.get("v1".to_string()), None);
        assert_eq!(cache.get("v2".to_string()), None);
        assert_eq!(cache.get("v3".to_string()), None);
        assert_eq!(cache.get("v4".to_string()), None);
        assert_eq!(cache.get("v5".to_string()), None);
        assert_eq!(cache.get("v6".to_string()), Some(6));
        assert_eq!(cache.get("v7".to_string()), Some(7));
        assert_eq!(cache.get("v8".to_string()), Some(8));
        assert_eq!(cache.get("v9".to_string()), Some(9));
        assert_eq!(cache.get("v10".to_string()), Some(10));
    }
}
