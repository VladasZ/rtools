use rtools::Dispatch;

#[tokio::main]
async fn main() {
    Dispatch::dispatch(
        async {
            return 5;
        },
        |_| {},
    );

    Dispatch::call();
    Dispatch::call();
    Dispatch::call();
}
