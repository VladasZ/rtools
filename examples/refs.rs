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
    let struc = Struct::default();
    let struc2 = Struct::default();

    mothod(&struc);
    mothod(&struc);
    mothod(&struc2);

    mothod(&Spok {});
    mothod(&Spok {});
}

fn mothod(val: &dyn Trait) {
    dbg!(to_data::<u64, _>(val));
}
