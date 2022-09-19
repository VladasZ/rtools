use std::ops::Range;

use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng, Rng,
};

pub trait Random: Sized {
    fn random() -> Self;
    fn random_in(range: Range<Self>) -> Self;
}

impl Random for String {
    fn random() -> Self {
        let mut rng = thread_rng();
        Alphanumeric.sample_string(&mut rng, 8)
    }
    fn random_in(_: Range<Self>) -> Self {
        unimplemented!()
    }
}

impl Random for i32 {
    fn random() -> Self {
        let mut rng = thread_rng();
        rng.gen_range(-100..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for u32 {
    fn random() -> Self {
        let mut rng = thread_rng();
        rng.gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for usize {
    fn random() -> Self {
        let mut rng = thread_rng();
        rng.gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}
