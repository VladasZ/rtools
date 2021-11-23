use std::{fs, path::Path};
use android_ndk_sys::AAsset_getLength;
use android_ndk_sys::AASSET_MODE_STREAMING;
use android_ndk_sys::AAssetManager_open;
use android_ndk_sys::AAssetManager_fromJava;
use android_ndk_sys::AAsset_read;
pub struct File {}

#[cfg(target_os = "android")]
static mut ASSET_MANAGER: *mut android_ndk_sys::AAssetManager = std::ptr::null_mut();

#[cfg(target_os = "android")]
pub fn set_asset_manager(env: android_ndk_sys::JNIEnv, asset_manager: android_ndk_sys::jobject) {
    unsafe { ASSET_MANAGER = AAssetManager_fromJava(env as _, asset_manager) }
}

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

    if ASSET_MANAGER.is_null() {
        error!("Android asset manager is null");
        panic!("Android asset manager is null");
    }

    let c_code = CString::new(path.as_ref().to_string_lossy().into_owned()).unwrap();

    let code_ptr = c_code.as_ptr();

    let asset = AAssetManager_open(ASSET_MANAGER, code_ptr, AASSET_MODE_STREAMING as _);
    if asset.is_null() {
        error!("Failed to open file: {:?}. Asset is null.", path.as_ref());
        panic!("Failed to open file: {:?}. Asset is null.", path.as_ref());
    }

    let size = AAsset_getLength(asset);
    let mut data: Vec<u8> = vec![0; size as _];
    AAsset_read(asset, data.as_mut_ptr() as _, size as _);
    let stro = std::str::from_utf8(&data).unwrap();
    Ok(stro.into())

}