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

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        let is_success = match self.store.insert(key.into(), value.into()) {
            Some(_) => Err("Key already exists in cache"),
            None => Ok(()),
        };

        return is_success;
    }

    pub fn get(&mut self, key: &str) -> Result<&String, &'static str> {
        let value = match self.store.get(key) {
            Some(v) => Ok(v),
            None => Err("Key does not exist in cache"),
        };

        return value;
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

        let key = "foo";
        let value = "123";

        // Add a key-value pair to the cache
        let result = cache.set(&key, &value);

        // Check that the method returns Ok
        assert_eq!(result, Ok(()));

        let second_value = "456";

        // Add another key-value pair with the same key
        let result = cache.set(&key, &second_value);

        // Check that the method returns an Err with the correct error message
        assert_eq!(result, Err("Key already exists in cache"));
    }

    #[test]
    fn test_get() {
        let mut cache = Cache::new(60, false);

        // Add a key-value pair to the cache
        cache.set("foo", "123").unwrap();

        // Get the value associated with the key
        let result = cache.get("foo");

        // Check that the method returns Ok with the correct value
        assert_eq!(result, Ok(&"123".to_string()));

        // Try to get the value associated with a non-existent key
        let result = cache.get("bar");

        // Check that the method returns an Err with the correct error message
        assert_eq!(result, Err("Key does not exist in cache"));
    }
}
