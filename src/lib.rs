mod extractors;
mod initializers;

pub use extractors::{handlers, Handlers};
pub use initializers::{edusko_job_spawner, Site};
use initializers::{ghanayello, goafricaonline_spawner};
use rayon::prelude::*;
use reqwest::Response;
use std::{collections::HashMap, sync::mpsc::Receiver};
use tokio::{
    sync::mpsc::{self, Sender},
    time::{sleep, Duration},
    try_join,
};
pub struct FetchedResult {
    pub response: Resp,
    pub handler: Handlers,
}

pub enum Resp {
    Result(String),
    Resp(Response),
}

/// helps find out about site information like:
/// number of pages so all pages url can be generated and scrapped asynchronously
/// pages `Site`s from spawned tasks are channeled down stream to request spawner
pub async fn job_spawner(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    let _ = try_join!(
        edusko_job_spawner(sender.clone()),
        // dummy(sender.clone()),
        // schoolcompass::extract_urls(sender.clone()),
        ghanayello::extract_urls(sender.clone()),
        goafricaonline_spawner(sender.clone())
    );
    Ok(())
}

/// fetch all pages received asynchronously and pass
/// response to handlers
pub async fn request_spawner(
    mut receiver: mpsc::Receiver<Site>,
    result_tx: mpsc::Sender<FetchedResult>,
) -> Result<(), ()> {
    let mut handlers = Vec::new();
    while let Some(site) = receiver.recv().await {
        let sender = result_tx.clone();
        let handler = tokio::task::spawn(async move {
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
                response: Resp::Resp(res),
                handler: site.handler,
            })
            .await;
    } else {
        drop(sender);
    }
}

pub fn process_data(input: Receiver<FetchedResult>) {
    let res: Vec<HashMap<&'static str, String>> = input
        .into_iter()
        .par_bridge()
        .map(|val| {
            let res = match val.response {
                Resp::Result(text) => text,
                _ => String::new(),
            };

            if res.is_empty() {
                return vec![];
            }

            match val.handler {
                Handlers::Edusko => handlers::edusko_data_extractor(&res),
                Handlers::Ghanayello => vec![handlers::ghanayello_data_extractor(&res)],

                Handlers::GoAfricaOnline => handlers::goafrica_data_extractor(&res),
                Handlers::SchoolCompass => vec![],
            }
        })
        .flat_map(|val| val)
        .collect();

    println!("{res:#?} = {}", res.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {}
}
