mod extractors;
mod initializers;

pub use extractors::Handlers;
use initializers::ghanayello;
pub use initializers::{dummy, edusko_job_spawner, Site};
use reqwest::Response;
use tokio::{
    sync::mpsc::{self, Sender},
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
        // edusko_job_spawner(sender.clone()),
        dummy(sender.clone()),
        // schoool_compass_spawner(sender.clone())
        ghanayello::extract_urls(sender.clone())
    );
    Ok(())
}

/// fetch all pages received asynchronously and pass
/// response to handlers
pub async fn request_spawner(
    mut receiver: mpsc::Receiver<Site>,
    result_tx: mpsc::Sender<FetchedResult>,
) -> Result<(), ()> {
    while let Some(site) = receiver.recv().await {
        let sender = result_tx.clone();
        tokio::task::spawn(async move {
            let ident = format!("{:?}", site);
            println!("\n=======================fetching_data for {:?}\n", ident);
            fetch_data(site, sender).await;
            println!("\n___________end of fetching for site: {}\n", ident);
        });
    }
    Ok(())
}

async fn fetch_data(site: Site, sender: Sender<FetchedResult>) {
    if let Ok(res) = reqwest::get(site.url).await {
        sender
            .send(FetchedResult {
                response: res,
                handler: site.handler,
            })
            .await
            .expect("channel buffer is probably full");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {}
}
