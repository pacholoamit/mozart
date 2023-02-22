pub struct Cache {
    /// the standard ttl as number in seconds for every generated cache element
    ttl: u32,
    ///
    delete_on_expire: bool,
}

impl Cache {
    pub fn new(ttl: u32, delete_on_expire: bool) -> Self {
        Cache {
            ttl,
            delete_on_expire,
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            ttl: 0,
            delete_on_expire: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cache() {
        let cache = Cache::new(60, true);

        assert_eq!(cache.ttl, 60);
        assert_eq!(cache.delete_on_expire, true)
    }

    #[test]
    fn test_default_cache() {
        let cache = Cache::default();

        assert_eq!(cache.ttl, 0);
        assert_eq!(cache.delete_on_expire, false)
    }
}
