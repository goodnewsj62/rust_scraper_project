use rust_scraper_project::{job_spawner, request_spawner};
use std::time::Instant;
use tokio::{sync::mpsc, try_join};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let mut count = 0u32;
    let (sender, receiver) = mpsc::channel(1000);
    let (result_tx, mut result_rx) = mpsc::channel(1000);

    let _ = try_join!(job_spawner(sender), request_spawner(receiver, result_tx));

    while let Some(message) = result_rx.recv().await {
        if let Ok(_) = message.response.text().await {
            println!("-------------finished processing----------");
            count += 1;
        }
    }

    let elapsed = now.elapsed();
    println!("fetched {} pages in  {:.2?}", count, elapsed);

    // use a thread pool to process data and save to db async
}
// strategies,  basic info struct,  runner, persistence
