#![allow(clippy::mismatched_target_os)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

cfg_if::cfg_if! { if #[cfg(android)] {
    use android_ndk_sys::AAsset_getLength;
    use android_ndk_sys::AASSET_MODE_STREAMING;
    use android_ndk_sys::AAssetManager_open;
    use android_ndk_sys::AAssetManager_fromJava;
    use android_ndk_sys::AAsset_read;
    use android_ndk_sys::AAsset_close;
}}

pub struct File {}

impl File {
    pub fn exists(path: impl AsRef<Path>) -> bool {
        path.as_ref().exists()
    }

    pub fn mkdir(path: impl AsRef<Path>) -> Result<()> {
        if File::exists(&path) {
            return Ok(());
        }
        fs::create_dir(path.as_ref())?;
        Ok(())
    }

    pub fn rm(path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if !File::exists(path) {
            return Ok(());
        }
        fs::remove_dir_all(path)?;
        Ok(())
    }

    pub fn get_files(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let files: Vec<_> = fs::read_dir(path)?.map(|a| Ok(a?.path())).collect::<Result<_>>()?;
        Ok(files)
    }

    pub fn ls() -> Vec<String> {
        fs::read_dir(".")
            .unwrap()
            .map(|f| f.unwrap().file_name().to_string_lossy().to_string())
            .collect()
    }
}

#[cfg(android)]
static mut ASSET_MANAGER: *mut android_ndk_sys::AAssetManager = std::ptr::null_mut();

#[cfg(android)]
#[mutants::skip]
pub fn set_asset_manager(env: android_ndk_sys::JNIEnv, asset_manager: android_ndk_sys::jobject) {
    unsafe { ASSET_MANAGER = AAssetManager_fromJava(env as _, asset_manager) }
}

impl File {
    pub fn read_to_string(path: impl AsRef<Path>) -> String {
        cfg_if::cfg_if! { if #[cfg(android)] {
            std::str::from_utf8(&unsafe { android_read(path) }).unwrap().to_string()
        } else {
            match fs::read_to_string(&path) {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to read file: {:?}, {}", path.as_ref(), err);
                    panic!("Failed to read file: {:?}, {}", path.as_ref(), err);
                }
            }
        }}
    }

    pub fn read(path: impl AsRef<Path>) -> Vec<u8> {
        cfg_if::cfg_if! { if #[cfg(android)] {
            unsafe { android_read(path) }
        } else {
            match fs::read(&path) {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to read file: {:?}, {}", path.as_ref(), err);
                    panic!("Failed to read file: {:?}, {}", path.as_ref(), err);
                }
            }
        }}
    }
}

#[cfg(android)]
pub unsafe fn android_read(path: impl AsRef<Path>) -> Vec<u8> {
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
    AAsset_close(asset);
    data
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashSet,
        path::{PathBuf, MAIN_SEPARATOR},
        str::FromStr,
    };

    use anyhow::Result;

    use crate::{file::File, random::Random};

    #[test]
    fn read() {
        let string = File::read_to_string("Cargo.toml");
        let data = File::read("Cargo.toml");

        assert_eq!(string, String::from_utf8(data).unwrap());
    }

    #[test]
    fn test() {
        assert_eq!(
            File::ls()
                .into_iter()
                .filter(|file| !file.contains("DS_Store"))
                .collect::<HashSet<_>>(),
            ["build.rs", "Cargo.toml", "src"]
                .into_iter()
                .map(ToString::to_string)
                .collect::<HashSet<_>>()
        );

        let prepend = format!(".{MAIN_SEPARATOR}");

        assert_eq!(
            File::get_files(".")
                .unwrap()
                .into_iter()
                .filter(|file| !file.to_string_lossy().contains("DS_Store"))
                .collect::<HashSet<_>>(),
            [
                PathBuf::from_str(&format!("{prepend}build.rs")).unwrap(),
                PathBuf::from_str(&format!("{prepend}Cargo.toml")).unwrap(),
                PathBuf::from_str(&format!("{prepend}src")).unwrap()
            ]
            .into_iter()
            .collect::<HashSet<_>>(),
        );

        assert!(File::exists("Cargo.toml"));
        assert_eq!(File::exists("fksdjflkdsj"), false);
    }

    #[test]
    fn mkdir_and_rm() -> Result<()> {
        let name = format!("../{}", String::random());

        assert_eq!(File::exists(&name), false);
        File::mkdir(&name)?;
        assert_eq!(File::exists(&name), true);
        File::rm(&name)?;
        assert_eq!(File::exists(&name), false);

        Ok(())
    }
}
