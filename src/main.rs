use rust_scraper_project::{job_spawner, request_spawner};
use std::time::Instant;
use tokio::{sync::mpsc, try_join};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let mut count = 0u32;
    let (sender, receiver) = mpsc::channel(50000);
    let (result_tx, mut result_rx) = mpsc::channel(50000);

    // let _ = job_spawner(sender.clone()).await;

    let _ = try_join!(job_spawner(sender), request_spawner(receiver, result_tx));

    while let Some(message) = result_rx.recv().await {
        if let Ok(_) = message.response.text().await {
            println!("-------------finished processing----------");
            count += 1;
        }
    }

    // while let Some(site) = receiver.recv().await {
    //     println!("{site:?}")
    //     // let sender = result_tx.clone();
    //     // tokio::task::spawn(async move {
    //     //     let ident = format!("{:?}", site);
    //     //     println!("\n=======================fetching_data for {:?}\n", ident);
    //     //     // fetch_data(site, None).await;
    //     //     println!("\n___________end of fetching for site: {}\n", ident);
    //     // });
    // }

    println!("#############################################################");

    let elapsed = now.elapsed();
    println!("fetched {} pages in  {:.2?}", count, elapsed);

    // use a thread pool to process data and save to db async
}
// strategies,  basic info struct,  runner, persistence
