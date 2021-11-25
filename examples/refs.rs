use std::ops::{Deref, DerefMut};

use tools::Property;

#[derive(Debug, Default)]
struct RefTest {
    pub sok: i32,
}

impl RefTest {
    fn get_rof(&mut self) -> &mut i32 {
        &mut self.sok
    }
}

impl Deref for RefTest {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.sok
    }
}

impl DerefMut for RefTest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sok
    }
}

fn main() {
    let mut spik = Property::<i32>::default();

    dbg!(&spik);

    spik.set(777);

    dbg!(&spik);

    let onto: i32 = spik.copy();

    dbg!(onto);

    println!("vscode pizdato");

    let mut sokil = RefTest::default();

    dbg!(&sokil);

    *sokil.get_rof() = 20;

    dbg!(&sokil);

    *sokil = 25;

    dbg!(&sokil);
}
