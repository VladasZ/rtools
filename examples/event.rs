use rtools::Event;

fn main() {
    let kok_changed = Event::<u32>::default();

    kok_changed.set(&(), |_, kok| {});

    kok_changed.trigger(500);
}
