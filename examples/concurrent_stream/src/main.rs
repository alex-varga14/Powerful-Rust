use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    stream::iter(0..200u64)
        .for_each_concurrent(20, |number| async move {
            let mut rng = thread_rng();
            let sleep_ms: u64 = rng.gen_range(0..20);
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            println!("{}", number);
        })
        .await;
}

// Lack of order shows that the jobs are being executed concurrently

// 3 Ways to use Streams to replace worker pools and collect the results in an idiomatic and functional way.
// ALWAYS PUT AN UPPER LIMIT ON THE NUMBER OF CONCURRENT TASKS
// OTHERWISE, MAY QUICKLY EXHAUST THE RESOURCES OF YOUR SYSTEM AND AFFECT PERFORMEANCE.