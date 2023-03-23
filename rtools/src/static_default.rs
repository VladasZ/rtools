#[macro_export]
macro_rules! static_default {
    ($type:ident) => {
        static mut _STATIC_DEFAULT: *mut $type = std::ptr::null_mut();
        impl $type {
            pub fn get() -> &'static mut $type {
                unsafe {
                    if _STATIC_DEFAULT.is_null() {
                        _STATIC_DEFAULT = Box::into_raw(Box::new($type::default()));
                    }
                    _STATIC_DEFAULT.as_mut().unwrap_unchecked()
                }
            }
        }
    };
}
