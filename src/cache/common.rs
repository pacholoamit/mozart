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
    fn get(&mut self, key: impl Into<String>) -> Option<Value>;
    fn get_multiple(&mut self, keys: Vec<impl Into<String>>) -> Vec<Value>;
    fn delete(&mut self, key: impl Into<String>);
    fn delete_multiple(&mut self, keys: Vec<impl Into<String>>);
    fn keys(&mut self) -> Vec<String>;
    fn has(&mut self, key: impl Into<String>) -> bool;
}
