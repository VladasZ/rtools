#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(trait_upcasting)]

trait Spilkog: SpilkogCallbacks {}

trait SpilkogCallbacks {
    fn setup(&self);
}

impl<T: ?Sized + Spilkog> SpilkogCallbacks for T {
    default fn setup(&self) {}
}

#[derive(Default)]
struct Strekol;
impl Spilkog for Strekol {}
impl SpilkogCallbacks for Strekol {
    fn setup(&self) {}
}

#[derive(Default)]
struct Izok;
impl Spilkog for Izok {}

fn main() {
    let sigo: &dyn Spilkog = &Strekol::default();
    let izo: &dyn Spilkog = &Izok::default();

    sigo.setup();
    izo.setup();
}
