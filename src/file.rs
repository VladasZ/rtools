use std::{fs, path::Path};
use android_ndk_sys::AAssetManager_open;
use android_ndk_sys::AASSET_MODE_STREAMING;
pub struct File {}

#[cfg(target_os = "android")]
pub static mut ASSET_MANAGER: *mut android_ndk_sys::AAssetManager = std::ptr::null_mut();

impl File {
    pub fn read_to_string(path: impl AsRef<Path>) -> std::io::Result<String> {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "android")] {
                unsafe { android_read_to_string(path) }
            } else {
                fs::read_to_string(path)
            }
        }
    }
}

#[cfg(target_os = "android")]
pub unsafe fn android_read_to_string(path: impl AsRef<Path>) -> std::io::Result<String> {
    use std::ffi::CString;

    //let sokel = Option::<android_ndk::android_app::AndroidApp>::default();

    if ASSET_MANAGER.is_null() {
        error!("Android asset manager is null");
        panic!("Android asset manager is null");
    }

    error!("sozdaju sudbu 2! {:?}" , path.as_ref());

    let c_code = CString::new(path.as_ref().to_string_lossy().into_owned()).unwrap();

    error!("path: {:?}", c_code);

    let code_ptr = c_code.as_ptr();

    AAssetManager_open(ASSET_MANAGER, code_ptr, AASSET_MODE_STREAMING as _);

    error!("sudba fergulna!");

    Ok("skigdok".into())

}