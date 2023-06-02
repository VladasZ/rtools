use std::time::Duration;

use tokio::{
    sync::mpsc::{self, Receiver},
    time::sleep,
};

pub struct Generator {
    value: u32,
}

impl Generator {
    pub fn generate(val: u32) -> Receiver<u32> {
        let (sender, receiver) = mpsc::channel::<u32>(1);

        tokio::spawn(async move {
            let mut genic = Generator { value: 0 };

            for _ in 0..val {
                genic.value += 1;

                sender.send(genic.value).await.expect("Failed to do this thing.");

                sleep(Duration::from_secs(1)).await;
            }
        });

        receiver
    }
}
