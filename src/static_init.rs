#[macro_export]
macro_rules! static_init {
    ($type:ident) => {
        static mut _STATIC_INIT: *mut $type = std::ptr::null_mut();
        impl $type {
            pub fn init(val: $type) {
                unsafe {
                    if !_STATIC_INIT.is_null() {
                        panic!("Double initialization of static get: {}", stringify!($type));
                    }
                    _STATIC_INIT = Box::into_raw(Box::new(val));
                }
            }

            pub fn get() -> &'static mut $type {
                unsafe {
                    if _STATIC_INIT.is_null() {
                        panic!("Uninitialized static init: {}", stringify!($type));
                    }
                    _STATIC_INIT.as_mut().unwrap_unchecked()
                }
            }

            pub fn is_ok() -> bool {
                unsafe { !_STATIC_INIT.is_null() }
            }
        }
    };
}
