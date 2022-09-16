use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng, Rng,
};

pub trait Random {
    fn random() -> Self;
}

impl Random for String {
    fn random() -> Self {
        let mut rng = thread_rng();
        Alphanumeric.sample_string(&mut rng, 8)
    }
}

impl Random for u32 {
    fn random() -> Self {
        let mut rng = thread_rng();
        rng.gen_range(0..100)
    }
}
