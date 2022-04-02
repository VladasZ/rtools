use rtools::Animation;

fn main() {
    let animation = Animation::new(1, 2, 10);

    loop {
        use std::{thread, time};
        let dur = time::Duration::from_millis(10);
        thread::sleep(dur);

        dbg!(animation.value());
    }
}
