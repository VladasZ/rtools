use serde::{Deserialize, Serialize};
use tools::{New, PropertyWrapper};

#[derive(Serialize, Deserialize, Debug)]
struct TestGest {
    pub i: u32,
    pub stro: String,
}

impl New for TestGest {
    fn new() -> TestGest {
        TestGest {
            i: 100100,
            stro: "rglo".into(),
        }
    }
}

fn main() {
    let mut wrapper = PropertyWrapper::<TestGest>::new("sokol");

    dbg!(&wrapper.i);
    dbg!(&wrapper.stro);

    wrapper.i += 1;
    wrapper.stro = "guga".into();

    wrapper.store();
}
