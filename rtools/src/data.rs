use std::{
    borrow::BorrowMut,
    mem::{size_of, zeroed},
};

pub fn to_data<Result: Copy, T>(value: T) -> Vec<Result> {
    let ptr: *const Result = std::ptr::from_ref::<T>(&value).cast::<Result>();
    let mut result = vec![];

    let size = size_of::<T>() / size_of::<Result>();

    for i in 0..size {
        result.push(unsafe { *ptr.add(i) });
    }

    result
}

pub fn to_bytes<T>(val: T) -> Vec<u8> {
    to_data(val)
}

pub fn from_bytes<T>(bytes: &[u8]) -> T {
    if bytes.len() != size_of::<T>() {
        panic!();
    }

    let mut val: T = unsafe { zeroed() };
    let mut ptr = std::ptr::from_mut::<T>(val.borrow_mut()).cast::<u8>();

    for byte in bytes {
        unsafe {
            *ptr = *byte;
            ptr = ptr.add(1);
        }
    }

    val
}

#[cfg(test)]
mod test {
    use crate::data::{from_bytes, to_bytes, to_data};

    #[test]
    fn test() {
        #[allow(dead_code)]
        #[derive(PartialEq, Debug, Clone)]
        struct A {
            a: u8,
            b: u8,
            c: u8,
        }

        let a = A { a: 10, b: 20, c: 30 };

        let bytes = to_bytes(a.clone());

        assert_eq!(vec![10, 20, 30], bytes);

        let from = from_bytes::<A>(&bytes);

        assert_eq!(from, a);
    }

    #[test]
    fn to_data_test() {
        #[allow(dead_code)]
        struct A {
            a: u32,
            b: u32,
            c: u32,
        }

        let a = A { a: 10, b: 20, c: 30 };

        let data: Vec<u32> = to_data(a);

        assert_eq!(data, [10, 20, 30]);

        #[allow(dead_code)]
        struct B {
            a: u8,
            b: u8,
            c: u8,
        }

        let b = B { a: 10, b: 20, c: 30 };

        let data: Vec<u8> = to_data(b);

        assert_eq!(data, [10, 20, 30]);
    }

    #[test]
    #[should_panic]
    fn wrong_size() {
        _ = from_bytes::<u8>(&[1, 2, 3, 4])
    }
}
