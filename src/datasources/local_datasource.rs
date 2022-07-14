use lru_time_cache::LruCache;
use std::fmt::Debug;
use std::time::Duration;

#[derive(Clone)]
pub struct LocalDatasource<T: Clone> {
    cache: LruCache<u8, Vec<T>>,
    time_cache: LruCache<u8, Vec<T>>,
}

impl<T> LocalDatasource<T>
where
    T: Clone + Debug,
{
    pub fn new(capacity: usize, duration: u64) -> Self {
        let expiry_duration = Duration::from_millis(duration);
        Self {
            cache: LruCache::<u8, Vec<T>>::with_capacity(capacity),
            time_cache: LruCache::<u8, Vec<T>>::with_expiry_duration_and_capacity(
                expiry_duration,
                capacity,
            ),
        }
    }

    pub fn insert(&mut self, value: Vec<T>, key: u8) {
        let timed_value = value.clone();
        self.cache.insert(key, value);
        self.time_cache.insert(key, timed_value);
    }

    pub fn get(&mut self, key: u8) -> Option<Vec<T>> {
        match self.cache.get(&key) {
            Some(value) => Some(value.clone()),
            None => match { self.time_cache.get(&key) } {
                Some(value) => {
                    self.cache.insert(key, value.clone());
                    Some(value.clone())
                }
                None => None,
            },
        }
    }

    pub fn get_recent(&mut self, key: u8) -> Option<&Vec<T>> {
        self.time_cache.get(&key)
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
        assert_eq!(cache.get(0_u8), Some(vec!["test".to_string()]));
    }

    #[test]
    fn test_get() {
        let mut cache = LocalDatasource::<String>::new(1, 1000);
        cache.insert(vec!["test".to_string()], 0_u8);
        assert_eq!(cache.get(0_u8), Some(vec!["test".to_string()]));
        assert_eq!(cache.get(0_u8), Some(vec!["test".to_string()]));
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
