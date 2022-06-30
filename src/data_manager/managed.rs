#[macro_export]
macro_rules! managed {
    ($type:ident) => {
        static mut PATH: *const std::path::PathBuf = std::ptr::null_mut();
        static mut STORAGE: *mut DataStorage<$type> = std::ptr::null_mut();

        impl Managed for $type {}

        impl DataManager<$type> for $type {
            fn path() -> &'static std::path::Path {
                unsafe { PATH.as_ref().unwrap() }
            }

            fn set_path(path: &std::path::Path) {
                unsafe {
                    PATH = Box::into_raw(Box::new(path.to_path_buf()));
                }
            }

            fn storage() -> &'static mut DataStorage<$type> {
                unsafe {
                    if STORAGE.is_null() {
                        STORAGE = Box::into_raw(Box::new(Default::default()));
                    }
                    STORAGE.as_mut().unwrap()
                }
            }
        }
    };
}
