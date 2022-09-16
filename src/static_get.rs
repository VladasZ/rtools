#[macro_export]
macro_rules! static_get {
    ($type:ident) => {
        static mut _STATIC_GET: *mut $type = std::ptr::null_mut();
        impl $type {
            pub fn get() -> &'static mut $type {
                unsafe {
                    if _STATIC_GET.is_null() {
                        _STATIC_GET = Box::into_raw(Box::new($type::default()));
                    }
                    _STATIC_GET.as_mut().unwrap()
                }
            }
        }
    };
}
