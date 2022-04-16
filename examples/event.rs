use rtools::Event;

fn main() {
    let kok_changed = Event::<u32>::default();

    kok_changed.subscribe((), |kok, _| {
        dbg!(kok + 1);
    });

    kok_changed.trigger(500);
}
