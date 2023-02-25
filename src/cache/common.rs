use crate::prelude::*;
use serde_json::Value;

pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: Value,
}

pub trait Cache {
    fn new(ttl: u32, delete_on_expire: bool) -> Self;
    fn set(&mut self, key: &str, value: &Value);
    fn set_multiple(&mut self, vec: Vec<KeyValue>);
    fn get(&mut self, key: &str) -> Result<Value, CacheError>;
}
