use mozart::cache::*;
use serde_json::json;

#[test]
fn test_cache_create() {
    let mut cache = Cache::create(Cache::HashMap);
    cache.set("wombo", &json!("combo"));
    assert_eq!(cache.get("wombo"), Some(&json!("combo")).cloned());
}
