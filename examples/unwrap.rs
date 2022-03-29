use rtools::Unwrap;

fn main() {
    let sol: Unwrap<i32> = 10.into();

    let sol2: Unwrap<i32> = Default::default();

    dbg!(&sol);
    dbg!(&sol2);

    //
    // dbg!("spek");
}
