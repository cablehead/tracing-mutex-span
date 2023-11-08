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

    pub fn with_lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.lock().unwrap();
        let child_span = tracing::info_span!("lock", name = %self.name);

        child_span.in_scope(|| f(&mut *guard))
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
