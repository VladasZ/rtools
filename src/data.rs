use std::{
    borrow::BorrowMut,
    mem::{size_of, zeroed},
};

pub fn to_bytes<T>(val: &T) -> Vec<u8> {
    let mut result = vec![];
    let mut ptr = val as *const T as *const u8;
    for _ in 0..size_of::<T>() {
        unsafe {
            result.push(*ptr);
            ptr = ptr.add(1);
        }
    }

    result
}

pub fn from_bytes<T>(bytes: &[u8]) -> T {
    if bytes.len() != size_of::<T>() {
        panic!();
    }

    let mut val: T = unsafe { zeroed() };
    let mut ptr = val.borrow_mut() as *mut T as *mut u8;

    for byte in bytes {
        unsafe {
            *ptr = *byte;
            ptr = ptr.add(1);
        }
    }

    val
}
