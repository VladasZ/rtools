extern crate tools;

use tools::*;

fn main() {
    let mut kok_changed = Event::<u32>::default();

    kok_changed.subscribe(|kok| {
        dbg!(kok + 1);
    });

    kok_changed.trigger(500);
}
