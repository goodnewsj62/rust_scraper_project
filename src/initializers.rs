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

pub mod ghanayello {
    use futures::TryFutureExt;
    use tokio::try_join;

    use super::*;

    pub async fn extract_urls(sender: mpsc::Sender<Site>) -> Result<(), ()> {
        let (trans, recv) = mpsc::unbounded_channel();

        let _ = page_spawner(trans.clone()).await;

        // let _ = try_join!(, );
        let _ = get_detail_url(sender, recv).await;

        Ok(())
    }

    async fn page_spawner(sender: mpsc::UnboundedSender<String>) -> Result<(), ()> {
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
                        sender.send(format!("{}{}/", url, n));
                    }
                }
            }
        });

        Ok(())
    }

    async fn get_detail_url(
        sender: mpsc::Sender<Site>,
        mut url_reciever: mpsc::UnboundedReceiver<String>,
    ) -> Result<(), ()> {
        while let Some(url) = url_reciever.recv().await {
            let sender_clone = sender.clone();
            task::spawn(async move {
                if let Ok(res) = reqwest::get(url).await {
                    if let Ok(html) = res.text().await {
                        let urls = {
                            let document = Html::parse_document(&html);

                            let scroller_selector =
                                Selector::parse(r#"div[class="company with_img g_0"]"#).unwrap();

                            document
                                .select(&scroller_selector)
                                .into_iter()
                                .filter_map(|div| {
                                    let anchor_selector = Selector::parse("a").unwrap();
                                    div.select(&anchor_selector).next()
                                })
                                .filter_map(|anchor| anchor.attr("href"))
                                .map(|href| format!("https://www.ghanayello.com{}", href))
                                .collect::<Vec<_>>()
                        };

                        for url in urls {
                            sender_clone
                                .send(Site {
                                    url,
                                    handler: Handlers::Ghanayello,
                                })
                                .await
                                .unwrap_or(())
                        }
                    }
                }
            });
        }

        Ok(())
    }
}

pub mod schoolcompass {
    use futures::TryFutureExt;
    use tokio::try_join;

    use super::*;

    pub async fn extract_urls(sender: mpsc::Sender<Site>) -> Result<(), ()> {
        // let (trans, recv) = mpsc::unbounded_channel();

        // let _ = try_join!(page_spawner(trans.clone()), get_detail_url(sender, recv));

        Ok(())
    }

    async fn page_spawner(sender: mpsc::UnboundedSender<String>) -> Result<(), ()> {
        let base_url = Arc::new("https://schoolscompass.com.ng/schools/");
        let school_types: Vec<String> = vec!["primary".into(), "secondary".into()];

        for school_type in school_types {
            let cloned_sender = sender.clone();
            let url = base_url.clone();

            task::spawn(async move {
                let url = format!("{}{}", *url, school_type);
                if let Ok(data) = reqwest::get(url.clone()).await {
                    if let Ok(html) = data.text().await {
                        let pages = {
                            let document = Html::parse_document(&html);

                            let scroller_selector =
                                Selector::parse(r#"a[class="page-link"]"#).unwrap();
                            document
                                .select(&scroller_selector)
                                .into_iter()
                                .flat_map(|v| v.text())
                                .filter_map(|text| text.trim().parse::<u32>().ok())
                                .last()
                                .expect("did not get a number")
                        };

                        for n in 0..pages {
                            let _ = cloned_sender.send(format!("{}/?page={}", url, n));
                        }
                    }
                }
            });
        }

        Ok(())
    }

    async fn schoool_compass_spawner(sender: mpsc::Sender<Site>) -> Result<(), ()> {
        Ok(())
    }
}

pub async fn goafricaonline_spawner(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    Ok(())
}
