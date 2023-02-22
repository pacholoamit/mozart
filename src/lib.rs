use std::collections::HashMap;

pub struct Cache {
    /// the standard ttl as number in seconds for every generated cache element
    ttl: u32,
    /// whether variables will be deleted automatically when they expire. If true the variable will be deleted. If false
    delete_on_expire: bool,
    store: HashMap<String, String>,
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            ttl: 0,
            delete_on_expire: false,
            store: HashMap::new(),
        }
    }
}

impl Cache {
    pub fn new(ttl: u32, delete_on_expire: bool) -> Self {
        Cache {
            ttl,
            delete_on_expire,
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<bool, &'static str> {
        let result = match self.store.insert(key, value) {
            Some(_) => Ok(true),
            None => Err("Key does not exit in cache"),
        };

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cache() {
        let cache = Cache::default();

        assert_eq!(cache.ttl, 0);
        assert_eq!(cache.delete_on_expire, false)
    }

    #[test]
    fn test_new_cache() {
        let cache = Cache::new(60, true);

        assert_eq!(cache.ttl, 60);
        assert_eq!(cache.delete_on_expire, true)
    }

    #[test]
    fn test_set() {
        let mut cache = Cache::default();

        // Add a key-value pair to the cache
        let result = cache.set("foo".to_string(), "123".to_string());

        // Check that the method returns Ok
        assert_eq!(result, Ok(true));

        // Add another key-value pair with the same key
        let result = cache.set("foo".to_string(), "456".to_string());

        // Check that the method returns an Err with the correct error message
        assert_eq!(result, Err("Key already exists in cache"));
    }
}
