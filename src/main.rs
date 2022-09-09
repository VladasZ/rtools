#![allow(clippy::mismatched_target_os)]

use rtools::passed::Passed;

fn main() {
    let mut pass = Passed::default();

    pass.print();
    pass.print();
}
