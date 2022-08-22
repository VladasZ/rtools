use rtools::bytes::to_data;

trait Trait {}

struct Spok {}

impl Trait for Spok {}

struct Struct {
    _val:  u16,
    _val2: u16,
    _val3: u16,
}

impl Default for Struct {
    fn default() -> Self {
        Self {
            _val:  1,
            _val2: 2,
            _val3: 3,
        }
    }
}

impl Trait for Struct {}

fn main() {
    let val = Struct::default();
    let val2 = Struct::default();

    mothod(&val);
    mothod(&val);
    mothod(&val2);

    mothod(&Spok {});
    mothod(&Spok {});
}

fn mothod(val: &dyn Trait) {
    dbg!(to_data::<u64, _>(val));
}
