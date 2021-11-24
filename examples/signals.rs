use std::io::Error;

use signal_hook::{consts::SIGINT, iterator::Signals};

use tokio::{
    task,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let ctrl_c = task::spawn(async {
        tokio::signal::ctrl_c().await.unwrap();
        dbg!("skidel");
    });

    let result = join.await?;

    dbg!(result);

    // Terminate the signal stream.

    dbg!("signals_task");
    ctrl_c.await?;

    dbg!("bye");

    Ok(())
}
