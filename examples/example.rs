use tokio::sync::broadcast;

use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use tracing_stacks::{fmt::write_entry, RootSpanLayer};

use tracing_mutex_span::TracingMutexSpan;

struct SharedState {}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(16);

    let logger = tokio::spawn(async move {
        let mut stdout = std::io::stdout();
        while let Ok(entry) = rx.recv().await {
            write_entry(&mut stdout, &entry).unwrap();
        }
    });

    {
        let _subscriber = tracing_subscriber::Registry::default()
            .with(RootSpanLayer::new(tx, None))
            .set_default();

        tracing::info!("let's go!");

        let mutex = TracingMutexSpan::new("SharedState", SharedState {});

        do_work(&mutex);
        do_work(&mutex);

        {
            let _guard = mutex.lock();
            info!("The shared state is locked and safe to access.");
        }

        info!("The program will now exit, releasing all locks if any remain.");
    }

    let _ = logger.await;
}

#[tracing::instrument(skip_all)]
fn do_work(mutex: &TracingMutexSpan<SharedState>) {
    let _guard = mutex.lock();
    info!("Locked and performing work on the shared state.");
}
