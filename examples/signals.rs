use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use futures::future::FutureExt as _;
use signal_hook::consts::SIGINT;
use tokio::{
    task,
    task::JoinHandle,
    time::{sleep, Duration},
};

struct SignalsHandler {
    cancelled: AtomicBool,
}

impl SignalsHandler {
    fn new() -> Arc<Self> {
        let new = Arc::new(Self {
            cancelled: Default::default(),
        });

        let moved = new.clone();
        let join = task::spawn(async move {
            loop {
                let cancelled = moved.cancelled.load(Ordering::Relaxed);

                dbg!(&cancelled);

                let ctrl_c = tokio::signal::ctrl_c();
                ctrl_c.await.unwrap();

                dbg!("OOO COTRO COO!!");

                moved.cancelled.store(true, Ordering::Relaxed);
            }
        });

        new
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = SignalsHandler::new();

    let join = task::spawn(async {
        let mut result = 0;
        for i in 0..20 {
            dbg!(i);
            dbg!(&result);
            result += i;
            sleep(Duration::from_secs(1)).await;
        }
        result
    });

    let result = join.await?;

    dbg!(result);

    Ok(())
}
