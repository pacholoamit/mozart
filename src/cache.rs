use std::collections::HashMap;

#[derive(Default)]
pub struct Cache {
    /// the standard ttl as number in seconds for every generated cache element
    ttl: u32,
    /// whether variables will be deleted automatically when they expire. If true the variable will be deleted. If false
    delete_on_expire: bool,
    store: HashMap<String, String>,
}

pub struct KeyValue<'a, T: ?Sized = str> {
    key: &'a str,
    value: &'a T,
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
        match self.store.insert(key.into(), value.into()) {
            Some(_) => Err("Key already exists in cache"),
            None => Ok(()),
        }
    }

    pub fn set_multiple(&mut self, vec: Vec<KeyValue>) -> Result<(), &'static str> {
        for item in vec.iter() {
            self.set(item.key, item.value).unwrap()
        }
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<&String, &'static str> {
        match self.store.get(key) {
            Some(v) => Ok(v),
            None => Err("Key does not exist in cache"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cache() {
        let cache = Cache::default();

        assert_eq!(cache.ttl, 0);
        assert!(!cache.delete_on_expire)
    }

    #[test]
    fn test_new_cache() {
        let cache = Cache::new(60, true);

        assert_eq!(cache.ttl, 60);
        assert!(cache.delete_on_expire);
    }

    #[test]
    fn test_set_value_in_cache() {
        let mut cache = Cache::default();

        let key = "foo";
        let value = "123";

        let result = cache.set(key, value);
        assert_eq!(result, Ok(()));

        let second_value = "456";
        let result = cache.set(key, second_value);
        assert_eq!(result, Err("Key already exists in cache"));
    }

    #[test]
    fn test_set_multiple() {
        let mut cache = Cache::default();

        let input = vec![
            KeyValue {
                key: "key1",
                value: "value1",
            },
            KeyValue {
                key: "key2",
                value: "value2",
            },
            KeyValue {
                key: "key3",
                value: "value3",
            },
        ];

        assert_eq!(cache.set_multiple(input), Ok(()));

        assert_eq!(cache.get("key1"), Ok(&String::from("value1")));
        assert_eq!(cache.get("key2"), Ok(&String::from("value2")));
        assert_eq!(cache.get("key3"), Ok(&String::from("value3")));
    }

    #[test]
    fn test_get_value_from_cache() {
        let mut cache = Cache::new(60, false);
        cache.set("foo", "123").unwrap();

        let result = cache.get("foo");
        assert_eq!(result, Ok(&"123".to_string()));

        let result = cache.get("bar");
        assert_eq!(result, Err("Key does not exist in cache"));
    }
}
