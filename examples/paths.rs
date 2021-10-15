use std::{env, path::PathBuf};

use path_absolutize::Absolutize;

fn main() {
    let dir = env::current_dir().unwrap();

    dbg!(&dir);
    dbg!(dir.absolutize().unwrap());

    dbg!(PathBuf::from("spok").absolutize().unwrap());
}
