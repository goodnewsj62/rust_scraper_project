mod extractors;
mod initializers;

use std::sync::{Arc, Mutex};

pub use extractors::Handlers;
pub use initializers::{dummy, edusko_job_spawner, Site};
use initializers::{ghanayello, schoolcompass};
use reqwest::Response;
use tokio::{
    sync::mpsc::{self, Sender},
    time::{sleep, Duration},
    try_join,
};

pub struct FetchedResult {
    pub response: Response,
    pub handler: Handlers,
}

/// helps find out about site information like:
/// number of pages so all pages url can be generated and scrapped asynchronously
/// pages `Site`s from spawned tasks are channeled down stream to request spawner
pub async fn job_spawner(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    let _ = try_join!(
        edusko_job_spawner(sender.clone()),
        // dummy(sender.clone()),
        schoolcompass::extract_urls(sender.clone()),
        ghanayello::extract_urls(sender.clone())
    );
    Ok(())
}

/// fetch all pages received asynchronously and pass
/// response to handlers
pub async fn request_spawner(
    mut receiver: mpsc::Receiver<Site>,
    result_tx: mpsc::Sender<FetchedResult>,
    count: &Arc<Mutex<u32>>,
) -> Result<(), ()> {
    let mut handlers = Vec::new();
    while let Some(site) = receiver.recv().await {
        let sender = result_tx.clone();
        let u = Arc::clone(count);
        let handler = tokio::task::spawn(async move {
            {
                let mut count = u.lock().expect("could not hold lock");
                *count += 1;
                println!("++++++++++++++++++++++++++++++++++++++++++{}", count);
            }
            let ident = format!("{:?}", site);

            if site.should_sleep {
                sleep(Duration::from_secs(15)).await;
            }

            fetch_data(site, sender).await;

            println!("\n___________end of fetching for site: {}\n", ident);
        });

        handlers.push(handler)
    }

    for handler in handlers {
        let _ = handler.await;
    }

    Ok(())
}

async fn fetch_data(site: Site, sender: Sender<FetchedResult>) {
    if let Ok(res) = reqwest::get(site.url).await {
        let _ = sender
            .send(FetchedResult {
                response: res,
                handler: site.handler,
            })
            .await;
    } else {
        drop(sender);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {}
}
