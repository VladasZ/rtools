use tokio::{
    task,
    time::{sleep, Duration},
};

use std::io::Error;

use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;

use futures::stream::StreamExt;


async fn handle_signals(signals: Signals) {
    let mut signals = signals.fuse();
    while let Some(signal) = signals.next().await {

        match signal {
            SIGHUP => {
                dbg!("SIGHUP");
            }
            SIGTERM => {
                dbg!("SIGTERM");
            },
            SIGINT => {
                dbg!("SIGINT");
            },
            SIGQUIT => {
                dbg!("SIGQUIT");
            },
            _ => unreachable!(),
        }
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let signals = Signals::new(&[
        SIGHUP,
        SIGTERM,
        SIGINT,
        SIGQUIT,
    ])?;

    let handle = signals.handle();
    let signals_task = tokio::spawn(handle_signals(signals));


    let join = task::spawn(async {
        let mut result = 0;
        for i in 0..10 {
            dbg!(i);
            dbg!(&result);
            result += i;
            sleep(Duration::from_secs(1)).await;
        }
        result
    });

    let result = join.await?;

    dbg!(result);

    // Terminate the signal stream.

    dbg!("handle");

    handle.close();
    dbg!("signals_task");
    signals_task.await?;

    dbg!("bye");

    Ok(())
}
