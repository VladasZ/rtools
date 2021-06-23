
pub struct Platform;

impl Platform {
    pub const MAC: bool = cfg!(target_os = "macos");
    pub const WIN: bool = cfg!(target_os = "windows");
    pub const IOS: bool = cfg!(target_os = "ios");
    pub const ANDROID: bool = cfg!(target_os = "android");
    pub const MOBILE: bool = Platform::IOS || Platform::ANDROID;
}