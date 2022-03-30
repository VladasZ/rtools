use backtrace::Backtrace;

pub fn backtrace() {
    let bt = Backtrace::new();
    println!("{:?}", bt);
}
