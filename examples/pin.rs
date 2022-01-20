use std::pin::Pin;

#[derive(Default)]
struct Spika {
    pub data:        u32,
    pub super_spika: Option<Pin<Box<Spika>>>,
    pub spiks:       Vec<Pin<Box<Spika>>>,
}

impl Spika {
    pub fn add_subspika(self: Pin<Box<Self>>, spika: Pin<Box<Spika>>) {}
}

fn main() {
    let mut a = Box::pin(Spika::default());

    let mut child = Box::pin(Spika::default());
}
