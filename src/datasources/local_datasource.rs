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

    pub fn insert(&mut self, value: Vec<T>, key: Option<u8>) {
        let key = key.unwrap_or_else(|| 0 as u8);
        let timed_value = value.clone();
        self.cache.insert(key, value);
        self.time_cache.insert(key, timed_value);
    }

    pub fn get(&mut self, key: Option<u8>) -> Option<Vec<T>> {
        let key = key.unwrap_or_else(|| 0 as u8);
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

    pub fn get_recent(&mut self, key: Option<u8>) -> Option<&Vec<T>> {
        let key = key.unwrap_or_else(|| 0 as u8);
        self.time_cache.get(&key).clone()
    }

    #[allow(dead_code)]
    fn persist(self) {
        todo!("persist to disk");
    }
}
