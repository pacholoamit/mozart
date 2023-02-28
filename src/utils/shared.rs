use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Shared<T>(Arc<Mutex<T>>);

impl<T> Shared<T> {
    pub fn new(t: T) -> Self {
        Self(Arc::new(Mutex::new(t)))
    }

    pub fn lock(&self) -> std::sync::MutexGuard<T> {
        self.0.lock().unwrap()
    }
}
