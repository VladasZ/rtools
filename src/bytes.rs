use std::mem::size_of;

pub fn to_data<Result: Copy, T>(value: T) -> Vec<Result> {
    let ptr: *const Result = (&value) as *const T as *const Result;
    let mut result = vec![];

    let size = size_of::<T>() / size_of::<Result>();

    for i in 0..size {
        result.push(unsafe { *ptr.offset(i as _) });
    }

    result
}

pub fn data_pointer<T>(value: T) -> u64 {
    unsafe { *((&value) as *const T as *const u64) }
}
