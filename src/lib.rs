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
        let guard = self.inner.lock().unwrap();
        trace!("{} locked", self.name);
        TracingGuard {
            name: self.name.clone(),
            _guard: guard,
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

use std::ops::{Deref, DerefMut};

impl<'a, T> Deref for TracingGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._guard
    }
}

impl<'a, T> DerefMut for TracingGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._guard
    }
}
