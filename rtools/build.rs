use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        android: { target_os = "android" },
        macos:   { target_os = "macos"   },
        linux:   { target_os = "linux"   },

        mobile: { any(target_os = "android", target_os = "ios") },
    }
}
