use std::sync::{Mutex, MutexGuard};
use tracing::{info_span, span::Entered};
use tracing::Span;

// Define your TracingMutex struct here
pub struct TracingMutex<T> {
    inner: Mutex<T>,
}

impl<T> TracingMutex<T> {
    pub fn new(data: T) -> Self {
        TracingMutex {
            inner: Mutex::new(data),
        }
    }

    pub fn lock(&self) -> TracingGuard<'_, T> {
        let span = info_span!("lock_acquired");
        let enter = span.enter();

        let guard = self.inner.lock().expect("Failed to acquire lock");

        // Store both the span and the Entered guard directly
        TracingGuard { guard, _enter: enter, _span: span }
    }
}

// Updated definition of TracingGuard to include the span
pub struct TracingGuard<'a, T> {
    guard: MutexGuard<'a, T>,
    _enter: Entered<'a>, // This will be the Entered guard, store it directly
    _span: Span, // Also store the span to extend its lifetime
}

impl<'a, T> Drop for TracingGuard<'a, T> {
    fn drop(&mut self) {
        // The span is exited automatically when _enter is dropped
    }
}
