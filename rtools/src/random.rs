use std::ops::Range;

use itertools::Itertools;
use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::SliceRandom,
    thread_rng, Rng,
};

pub trait Random<T = Self>: Sized {
    fn random() -> Self {
        unimplemented!()
    }

    fn random_count(_: usize) -> Self {
        unimplemented!()
    }

    fn random_in(_: Range<Self>) -> Self {
        unimplemented!()
    }
}

pub trait RandomContainer<T>: Sized {
    fn random_member(&self) -> &T {
        unimplemented!()
    }
    fn take_random(&mut self) -> T {
        unimplemented!()
    }
}

impl Random for bool {
    fn random() -> Self {
        let mut rng = thread_rng();
        rng.gen_range(0..=1) != 0
    }
}

impl Random for char {
    fn random() -> Self {
        thread_rng().gen()
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for String {
    fn random() -> Self {
        let mut rng = thread_rng();
        Alphanumeric.sample_string(&mut rng, 8)
    }
}

impl Random for u8 {
    fn random() -> Self {
        thread_rng().gen()
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for i32 {
    fn random() -> Self {
        thread_rng().gen_range(-100..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for u64 {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for i64 {
    fn random() -> Self {
        thread_rng().gen_range(-100..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for u32 {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for isize {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for usize {
    fn random() -> Self {
        thread_rng().gen_range(0..100)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for f32 {
    fn random() -> Self {
        thread_rng().gen_range(0.0..100.0)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl Random for f64 {
    fn random() -> Self {
        thread_rng().gen_range(0.0..100.0)
    }

    fn random_in(range: Range<Self>) -> Self {
        thread_rng().gen_range(range)
    }
}

impl<T: Random<T>> Random<Vec<T>> for Vec<T> {
    fn random() -> Self {
        (0..usize::random_in(5..10)).map(|_| T::random()).collect_vec()
    }

    fn random_count(count: usize) -> Self {
        (0..count).map(|_| T::random()).collect_vec()
    }
}

impl<T> RandomContainer<T> for Vec<T> {
    fn random_member(&self) -> &T {
        self.choose(&mut thread_rng()).unwrap()
    }

    fn take_random(&mut self) -> T {
        self.remove(usize::random_in(0..self.len()))
    }
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use itertools::Itertools;

    use crate::{random::RandomContainer, Random};

    fn test_random_type<T: Random + PartialEq>() {
        let data = (0..10).map(|_| T::random()).collect_vec();
        let first = data.first().unwrap();

        assert_eq!(data.iter().all(|a| *a == *first), false);
    }

    #[test]
    #[should_panic]
    fn test_random_type_test() {
        #[derive(PartialEq)]
        struct NotRandom {
            a: i32,
        }

        impl Random for NotRandom {
            fn random() -> Self {
                NotRandom { a: 10 }
            }
        }

        test_random_type::<NotRandom>()
    }

    #[test]
    fn random_test() {
        test_random_type::<bool>();
        test_random_type::<char>();
        test_random_type::<u8>();
        test_random_type::<u32>();
        test_random_type::<i32>();
        test_random_type::<u64>();
        test_random_type::<i64>();
        test_random_type::<isize>();
        test_random_type::<usize>();
        test_random_type::<f32>();
        test_random_type::<f64>();
        test_random_type::<char>();
        test_random_type::<String>();
        test_random_type::<Vec<u32>>();
    }

    #[test]
    fn random_in_test() {
        fn test_type<T: Random + Clone + PartialOrd>(range: Range<T>) {
            for _ in 0..1000 {
                let ran = T::random_in(range.clone());
                assert!(range.contains(&ran));
            }
        }

        test_type::<char>('a'..'z');
        test_type::<u8>(5..100);
        test_type::<u32>(5..100);
        test_type::<i32>(5..100);
        test_type::<u64>(5..100);
        test_type::<i64>(5..100);
        test_type::<isize>(5..100);
        test_type::<usize>(5..100);
        test_type::<f32>(5.0..100.0);
        test_type::<f64>(5.0..100.0);
    }

    #[test]
    fn random_vec_elem() {
        for _ in 0..100 {
            let ve = vec![1, 2, 3, 4, 5];
            let r = ve.random_member();
            assert!(ve.contains(r));
        }
    }

    #[test]
    fn take_random() {
        for _ in 0..100 {
            let mut ve = vec![1, 2, 3, 4, 5, 6, 7, 8];
            let val = ve.take_random();
            assert!(!ve.contains(&val));
        }
    }

    #[test]
    fn random_ver_gen() {
        let random = Vec::<u32>::random();
        dbg!(&random);
        let random_count = Vec::<u32>::random_count(10);
        dbg!(&random_count);
        assert_eq!(random_count.len(), 10);
    }
}
