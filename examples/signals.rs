use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use tokio::{
    task,
    time::{sleep, Duration},
};

struct SignalsHandler {
    cancelled: AtomicU64,
}

impl SignalsHandler {
    fn new() -> Arc<Self> {
        let new = Arc::new(Self {
            cancelled: 0.into(),
        });

        let moved = new.clone();
        task::spawn(async move {
            loop {
                let cancelled = moved.cancelled.load(Ordering::Relaxed);

                let ctrl_c = tokio::signal::ctrl_c();
                ctrl_c.await.unwrap();

                moved.cancelled.store(cancelled + 1, Ordering::Relaxed);
            }
        });

        new
    }
}

async fn slot() -> Result<(), Box<dyn std::error::Error>> {
    let handler = SignalsHandler::new();

    let join = task::spawn(async move {
        let mut result = 0;
        for i in 0..200000 {
            result += i;
            sleep(Duration::from_secs(1)).await;
        }
        result
    });

    let result = join.await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    slot().await?;

    Ok(())
}
