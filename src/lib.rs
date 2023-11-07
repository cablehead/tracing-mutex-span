use std::sync::{Mutex, MutexGuard};

use tracing::trace;

pub struct TracingMutexSpan<T> {
    name: String,
    inner: Mutex<T>,
}

impl<T> TracingMutexSpan<T> {
    pub fn new(name: &str, data: T) -> Self {
        TracingMutexSpan {
            name: name.to_string(),
            inner: Mutex::new(data),
        }
    }

    pub fn lock(&self) -> TracingGuard<'_, T> {
        trace!("{} locked", self.name);
        TracingGuard {
            name: self.name.clone(),
            _guard: self.inner.lock().expect("Failed to acquire lock"),
        }
    }
}

pub struct TracingGuard<'a, T> {
    name: String,
    _guard: MutexGuard<'a, T>,
}

impl<'a, T> Drop for TracingGuard<'a, T> {
    fn drop(&mut self) {
        trace!("{} unlocked", self.name);
    }
}
