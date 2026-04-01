use std::sync::{Arc, RwLock};

pub type Shared<T> = Arc<RwLock<T>>;

pub fn new_shared<T>(obj: T) -> Shared<T> {
    Arc::new(RwLock::new(obj))
}
