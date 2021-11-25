use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
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
    cancelled: AtomicU64,
}

impl SignalsHandler {
    fn new() -> Arc<Self> {
        let new = Arc::new(Self {
            cancelled: 0.into(),
        });

        let moved = new.clone();
        let join = task::spawn(async move {
            loop {
                let cancelled = moved.cancelled.load(Ordering::Relaxed);

                dbg!(&cancelled);

                let ctrl_c = tokio::signal::ctrl_c();
                ctrl_c.await.unwrap();

                dbg!("OOO COTRO COO!!");

                moved.cancelled.store(cancelled + 1, Ordering::Relaxed);
            }
        });

        new
    }
}

const signals: &[wintrap::Signal] = &[
    wintrap::Signal::CtrlC,
    wintrap::Signal::CloseWindow,
    wintrap::Signal::CloseConsole,
    wintrap::Signal::CtrlBreak,
];

async fn slot() -> Result<(), Box<dyn std::error::Error>> {
    let handler = SignalsHandler::new();

    let join = task::spawn(async move {
        let mut result = 0;
        for i in 0..200000 {
            result += i;
            dbg!(handler.cancelled.load(Ordering::Relaxed));
            sleep(Duration::from_secs(1)).await;
        }
        result
    });

    let result = join.await?;

    dbg!(result);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let forg = wintrap::trap(
        &signals,
        |signal| {
            // handle signal here
            println!("Caught a signal: {:?}", signal);
            unsafe { windows::Win32::System::Diagnostics::Debug::Beep(2000, 50) };
        },
        || {
            println!("Doing work");
            // do work
            for i in 0..200000 {
                //  result += i;
                // dbg!(handler.cancelled.load(Ordering::Relaxed));
                std::thread::sleep(Duration::from_secs(1));
                //sleep(Duration::from_secs(1)).await;
            }
            println!("Doing work");
        },
    )
    .unwrap();

    slot().await;

    Ok(())
}
