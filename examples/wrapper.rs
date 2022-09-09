use rtools::Persistent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestGest {
    pub i:    u32,
    pub stro: String,
}

impl Default for TestGest {
    fn default() -> TestGest {
        TestGest {
            i:    100100,
            stro: "rglo".into(),
        }
    }
}

fn main() {
    let mut wrapper = Persistent::<TestGest>::new("sokol");

    wrapper.i += 1;
    wrapper.stro = "guga".into();

    wrapper.store();
}
