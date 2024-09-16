mod entities;
mod extractors;
mod initializers;

use core::str;
use entities::*;
pub use extractors::{handlers, Handlers};
pub use initializers::{edusko_job_spawner, Site};
use initializers::{ghanayello, goafricaonline_spawner};
use rayon::prelude::*;
use reqwest::Response;
use sea_orm::*;
use sha2::{Digest, Sha256};
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
    // schoolcompass::extract_urls(sender).await;
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

pub fn process_data(input: Receiver<FetchedResult>) -> Vec<HashMap<&'static str, String>> {
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
                Handlers::SchoolCompass => vec![handlers::school_compass_extractor(&res)],
            }
        })
        .flat_map(|val| val)
        .collect();

    println!("{}", res.len());
    res
}

pub async fn save_to_db(uri: &str, data: HashMap<&'static str, String>) -> Result<(), DbErr> {
    if data.contains_key(&"name")
        && data.contains_key(&"location")
        && data.contains_key(&"country")
        && data.contains_key(&"level")
    {
        let name = data
            .get("name")
            .expect("check spelling or check")
            .to_owned();
        let location = data
            .get("location")
            .expect("check spelling or check")
            .to_owned();
        let country = data
            .get("country")
            .expect("check spelling or check")
            .to_owned();

        let level = data
            .get("level")
            .expect("check spelling or check")
            .to_owned();

        let hash = get_hash(&format!("{}-{}", name, level));

        let db = Database::connect(uri).await?;

        // let sad_bakery: Option<school_data::Model> = school_data::Entity::find()
        //     .filter(school_data::Column::NameHash.eq(&hash))
        //     .filter(school_data::Column::level.eq(&))
        //     .one(db)
        //     .await?;

        let school = school_data::ActiveModel {
            school_name: ActiveValue::Set(name),
            location: ActiveValue::Set(location),
            name_hash: ActiveValue::Set(hash),
            country: ActiveValue::Set(country),
            school_type: ActiveValue::Set(level),
            logo: ActiveValue::Set(data.get("logo").cloned()),
            algorithm: ActiveValue::Set("sha256".to_owned()),
            city: ActiveValue::Set(data.get("city").cloned()),
            ..Default::default()
        };

        school.insert(&db).await?;
    }

    Ok(())
}

pub fn get_hash(value: &str) -> String {
    format!("{:x}", Sha256::digest(value))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {}
}
