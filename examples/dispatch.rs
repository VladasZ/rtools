use rtools::Dispatch;

#[tokio::main]
async fn main() {
    Dispatch::dispatch(
        async {
            return 5;
        },
        |value| {
            dbg!(value);
        },
    );

    Dispatch::call();
    Dispatch::call();
    Dispatch::call();
}
