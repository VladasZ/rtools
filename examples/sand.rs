trait HelloSayer {
    fn say_hello(&self) {
        println!("hello")
    }
}

trait Greeter: HelloSayer {}

impl<T> HelloSayer for T where T: Greeter {}

struct BrokenGreeter;

impl Greeter for BrokenGreeter {
    // fn say_hello() { // Shouldn't compile
    //     print!("bye bye!")
    // }
}

fn main() {
    let gr = BrokenGreeter;

    gr.say_hello();

    // dbg!("hello");
}
