use tools::data::{from_bytes, to_bytes};

#[derive(Debug)]
struct Test {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            a: 11,
            b: 22,
            c: 33,
            d: 44,
        }
    }
}

fn main() {
    dbg!(to_bytes(&Test::default()));

    dbg!(from_bytes::<Test>(&vec![11, 22, 33, 44]));
}
