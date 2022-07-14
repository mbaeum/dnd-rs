use lru_time_cache::LruCache;
use std::fmt::Debug;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct LocalDatasource<T> {
    cache: LruCache<u8, Vec<T>>,
    expiry_cache: LruCache<u8, Option<Instant>>,
}

impl<T> LocalDatasource<T>
where
    T: Debug,
{
    pub fn new(capacity: usize, duration: u64) -> Self {
        let expiry_date = Instant::now().checked_add(Duration::from_millis(duration));
        let mut expiry_cache = LruCache::<u8, Option<Instant>>::with_capacity(1);
        expiry_cache.insert(0, expiry_date);
        Self {
            cache: LruCache::<u8, Vec<T>>::with_capacity(capacity),
            expiry_cache,
        }
    }

    pub fn insert(&mut self, value: Vec<T>, key: u8) {
        self.cache.insert(key, value);
    }

    pub fn get(&mut self, key: u8) -> Option<&Vec<T>> {
        self.cache.get(&key)
    }

    pub fn get_recent(&mut self, key: u8) -> Option<&Vec<T>> {
        let now = Instant::now();
        if now <= self.expiry_cache.get(&0_u8)?.unwrap() {
            self.cache.get(&key)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn persist(self) {
        todo!("persist to disk");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut cache = LocalDatasource::<String>::new(1, 1000);
        cache.insert(vec!["test".to_string()], 0_u8);
        assert_eq!(cache.get(0_u8), Some(&vec!["test".to_string()]));
    }

    #[test]
    fn test_get() {
        let mut cache = LocalDatasource::<String>::new(1, 1000);
        cache.insert(vec!["test".to_string()], 0_u8);
        assert_eq!(cache.get(0_u8), Some(&vec!["test".to_string()]));
        assert_eq!(cache.get(0_u8), Some(&vec!["test".to_string()]));
    }

    #[test]
    fn test_get_recent() {
        let mut cache = LocalDatasource::<String>::new(1, 1000);
        cache.insert(vec!["test".to_string()], 0_u8);
        assert_eq!(cache.get_recent(0_u8), Some(&vec!["test".to_string()]));
    }

    #[test]
    fn test_get_recent_expires() {
        let mut cache = LocalDatasource::<String>::new(1, 2);
        cache.insert(vec!["test".to_string()], 0_u8);
        std::thread::sleep(Duration::from_millis(3));
        assert_eq!(cache.get_recent(0_u8), None);
    }
}
