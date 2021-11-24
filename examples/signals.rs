// use tokio::{
//     task,
//     time::{sleep, Duration},
// };

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let join = task::spawn(async {
//         let mut result = 0;
//         for i in 0..100 {
//             dbg!(i);
//             dbg!(&result);
//             result += i;
//             sleep(Duration::from_millis(100)).await;
//         }
//         result
//     });

//     let result = join.await?;

//     Ok(())
// }

fn main() {}
