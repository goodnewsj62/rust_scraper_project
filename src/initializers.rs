use std::sync::Arc;

use scraper::{ElementRef, Html, Selector};
use tokio::{sync::mpsc, task};

use crate::Handlers;

mod schema;

#[derive(Debug)]
pub struct Site {
    pub url: String,
    pub handler: Handlers,
}

pub async fn dummy(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    Ok(())
}

pub async fn edusko_job_spawner(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    use schema::Root;

    let base_url = Arc::new("https://api.edusko.com/api/v1/school/");
    let params: Vec<String> = vec!["nigeria".into(), "ghana".into(), "kenya".into()];

    for country in params {
        let owned_url = Arc::clone(&base_url);
        let cloned_snd = sender.clone();

        task::spawn(async move {
            let url = format!("{}?country={}&limit=50&page=1", *owned_url, country);
            let data = reqwest::get(url).await;
            //TODO: try again incase it fails

            if let Ok(data) = data {
                let res = data.json::<Root>().await;
                println!(
                    "\n\n---------------------doing job for {} ----------------------\n\n",
                    country
                );
                if let Ok(res) = res {
                    let total_pages = res.data.total_pages;

                    for page in 0..total_pages {
                        cloned_snd
                            .send(Site {
                                url: format!(
                                    "{}?country={}&limit=50&page={}",
                                    *owned_url, country, page
                                ),
                                handler: Handlers::Edusko,
                            })
                            .await
                            .unwrap_or(());
                    }

                    println!(
                        "\n=======================finished job==============pages: {}\n",
                        total_pages
                    );
                }
            }
        });
    }
    // request async to site
    // await and then construct
    Ok(())
}

pub async fn ghanayello_pages_url_constructor(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    task::spawn(async move {
        let url = "https://www.ghanayello.com/companies/find_a_school/";
        let data = reqwest::get(url).await;
        if let Ok(data) = data {
            if let Ok(html) = data.text().await {
                let pages = {
                    let document = Html::parse_document(&html);

                    let scroller_selector = Selector::parse("div.scroller_with_ul").unwrap();
                    let div_area = document
                        .select(&scroller_selector)
                        .next()
                        .expect("ghanayello site structure must have changed");

                    let ul = div_area
                        .first_child()
                        .expect("ghanayello site structure must have changed");

                    ul.children()
                        .filter_map(|li| ElementRef::wrap(li))
                        .flat_map(|v| v.text())
                        .filter_map(|value| value.parse::<u32>().ok())
                        .collect::<Vec<_>>()
                };

                for n in pages {
                    sender
                        .send(Site {
                            url: format!("{}{}/", url, n),
                            handler: Handlers::Ghanayello,
                        })
                        .await
                        .unwrap_or(());
                }
            }
        }
    });

    Ok(())
}
