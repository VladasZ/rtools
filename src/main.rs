#![allow(clippy::mismatched_target_os)]

use rtools::{passed::Passed, Random};

fn main() {
    let mut pass = Passed::default();

    pass.print();
    pass.print();

    for _ in 0..100 {
        dbg!(bool::random());
    }
}
