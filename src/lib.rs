use std::sync::{Mutex, MutexGuard};

use tracing::{info_span, Span};

pub struct TracingMutexSpan<T> {
    inner: Mutex<T>,
}

impl<T> TracingMutexSpan<T> {
    pub fn new(data: T) -> Self {
        TracingMutexSpan {
            inner: Mutex::new(data),
        }
    }

    pub fn lock(&self) -> TracingGuard<'_, T> {
        let span = info_span!("lock_acquired");
        let _guard = self.inner.lock().expect("Failed to acquire lock");
        span.in_scope(|| {}); // This is just to register the span
        TracingGuard {
            guard: _guard,
            span, // Move span into TracingGuard
        }
    }
}

pub struct TracingGuard<'a, T> {
    guard: MutexGuard<'a, T>,
    span: Span, // Store the span itself
}

impl<'a, T> Drop for TracingGuard<'a, T> {
    fn drop(&mut self) {
        self.span.in_scope(|| {
            tracing::info!("lock_released"); // Log that the lock has been released
        });
        // Exiting the scope will automatically exit the span
    }
}

use std::ops::{Deref, DerefMut};

impl<'a, T> Deref for TracingGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> DerefMut for TracingGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}
