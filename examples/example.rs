// examples/example.rs
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tracing_mutex_span::TracingMutex; // Make sure to use the actual name of your crate here.

struct SharedState {
    data: u64,
}

fn main() {
    // Set up the subscriber for tracing events.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Create a `TracingMutex` to protect the `SharedState`.
    let mutex = TracingMutex::new(SharedState { data: 42 });

    // Simulate doing some work with the locked state.
    do_work(&mutex);

    // Simulate more work in a separate scope, causing the lock to be acquired and released again.
    {
        let _guard = mutex.lock();
        info!("The shared state is locked and safe to access.");
        // Perform operations on the locked state here...
    } // The lock is automatically released here.

    info!("The program will now exit, releasing all locks if any remain.");
}

// A function that takes a `TracingMutex` and performs some work while the lock is held.
fn do_work(mutex: &TracingMutex<SharedState>) {
    let _guard = mutex.lock();
    info!("Locked and performing work on the shared state.");
    // Perform some work with the locked state here...
} // The lock is automatically released when `_guard` goes out of scope.