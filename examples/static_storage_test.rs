use rtools::static_storage;

static_storage!(Meslo, i32);
static_storage!(Kreslo, i32, 77);

fn main() {
    dbg!(Meslo::get());
    Meslo::set(20);
    dbg!(Meslo::get());

    dbg!(Kreslo::get());
    Kreslo::set(25);
    dbg!(Kreslo::get());
}
