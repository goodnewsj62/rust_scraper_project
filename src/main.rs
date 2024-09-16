mod config;

use config as conf;
use rust_scraper_project::{
    job_spawner, process_data, request_spawner, save_to_db, FetchedResult, Resp,
};
use std::sync::mpsc as syncmpsc;
use std::time::Instant;

use tokio::{sync::mpsc, try_join};

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let mut count = 0u32;
    let app_config = conf::Config::build();
    let (sender, receiver) = mpsc::channel(50000);
    let (result_tx, mut result_rx) = mpsc::channel(50000);
    let (send_process, rx_process) = syncmpsc::channel();

    let _ = try_join!(job_spawner(sender), request_spawner(receiver, result_tx));

    while let Some(message_) = result_rx.recv().await {
        count += 1;
        let message = match message_.response {
            Resp::Resp(val) => val.text().await,
            _ => Ok(String::from("")),
        };

        if let Ok(message) = message {
            match send_process.send(FetchedResult {
                response: Resp::Result(message),
                handler: message_.handler,
            }) {
                Err(err) => println!("{err:?}"),
                _ => println!("sent..."),
            }
        }
    }

    drop(send_process);

    let extract = process_data(rx_process);

    for data in extract {
        let _ = save_to_db(&app_config.db_uri, data).await;
    }

    println!("#############################################################");

    let elapsed = now.elapsed();
    println!("fetched {} pages in  {:.2?}", count, elapsed);

    // use a thread pool to process data and save to db async
}
// strategies,  basic info struct,  runner, persistence
