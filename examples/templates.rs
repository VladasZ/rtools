#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]

use std::marker::PhantomData;

struct Spekul<In = (), Out = ()> {
    _data: PhantomData<(In, Out)>,
}

impl<In, Out> Spekul<In, Out> {}

pub struct Goleg;

pub trait Sokolina {
    type On = ();
    type Eut = ();
}

impl Sokolina for Goleg {}

type Gotel<T> = Spekul<(), T>;

fn main() {
    let _spokolok = Gotel::<f32> {
        _data: Default::default(),
    };
}
