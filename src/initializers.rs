use scraper::{selectable::Selectable, ElementRef, Html, Selector};
use tokio::{sync::mpsc, task};

use crate::Handlers;

pub mod schema;

#[derive(Debug)]
pub struct Site {
    pub url: String,
    pub handler: Handlers,
    pub should_sleep: bool,
}

// pub async fn dummy(sender: mpsc::Sender<Site>) -> Result<(), ()> {
//     Ok(())
// }

pub async fn edusko_job_spawner(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    use schema::Root;

    let params: [String; 3] = ["nigeria".into(), "ghana".into(), "kenya".into()];
    // let mut handlers = Vec::new();

    for country in params {
        let owned_url = "https://api.edusko.com/api/v1/school/";
        let cloned_snd = sender.clone();

        task::spawn(async move {
            let url = format!("{}?country={}&limit=50&page=1", owned_url, country);
            //TODO: try again incase it request fails

            if let Ok(data) = reqwest::get(url).await {
                let res = data.json::<Root>().await;
                println!(
                    "\n\n---------------------doing job for {} ----------------------\n\n",
                    country
                );
                if let Ok(res) = res {
                    let total_pages = res.data.total_pages;

                    for page in 1..total_pages {
                        cloned_snd
                            .send(Site {
                                url: format!(
                                    "{}?country={}&limit=50&page={}",
                                    owned_url, country, page,
                                ),
                                handler: Handlers::Edusko,
                                should_sleep: false,
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

    Ok(())
}

pub mod ghanayello {
    use tokio::try_join;

    use super::*;

    pub async fn extract_urls(sender: mpsc::Sender<Site>) -> Result<(), ()> {
        let (trans, recv) = mpsc::unbounded_channel();

        let _ = try_join!(page_spawner(trans), get_detail_url(sender, recv));

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
                            .filter_map(ElementRef::wrap)
                            .flat_map(|v| v.text())
                            .filter_map(|value| value.parse::<u32>().ok())
                            .collect::<Vec<_>>()
                    };

                    for n in pages {
                        let _ = sender.send(format!("{}{}/", url, n));
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
        let mut hanlers = Vec::new();

        while let Some(url) = url_reciever.recv().await {
            let sender_clone = sender.clone();
            let handler = task::spawn(async move {
                if let Ok(res) = reqwest::get(url).await {
                    if let Ok(html) = res.text().await {
                        let urls = {
                            let document = Html::parse_document(&html);

                            let scroller_selector =
                                Selector::parse(r#"div[class="company with_img g_0"]"#).unwrap();

                            document
                                .select(&scroller_selector)
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
                                    should_sleep: false,
                                })
                                .await
                                .unwrap_or(())
                        }
                    }
                }
            });

            hanlers.push(handler);
        }

        for handler in hanlers {
            let _ = handler.await;
        }
        Ok(())
    }
}

pub async fn goafricaonline_spawner(sender: mpsc::Sender<Site>) -> Result<(), ()> {
    let school_types = ["ecoles-secondaires", "ecoles-primaires"];

    for school_type in school_types {
        let sender = sender.clone();
        let base_url = "https://www.goafricaonline.com/cm/annuaire/";
        task::spawn(async move {
            let url = format!("{}{}", base_url, school_type);
            if let Ok(res) = reqwest::get(url.clone()).await {
                if let Ok(data) = res.text().await {
                    let pages = {
                        let document = scraper::Html::parse_document(&data);
                        let selector =
                            Selector::parse(r#"ul[class="pagination"]>li"#).expect("html changed");
                        document
                            .select(&selector)
                            .flat_map(|val| val.text())
                            .filter_map(|text| text.trim().parse::<u32>().ok())
                            .last()
                            .expect("issue with goafricaonline")
                    };

                    println!("====================p{}", pages);

                    for page in 1..(pages + 1) {
                        sender
                            .send(Site {
                                url: format!("{}?p={page}", url),
                                handler: Handlers::GoAfricaOnline,
                                should_sleep: false,
                            })
                            .await
                            .unwrap_or_else(|_| {
                                println!("xxxx an error occurred sending a value in goafrica xxxx")
                            });
                    }
                }
            }
        });
    }

    Ok(())
}

//pub mod schoolcompass {
//     use tokio::try_join;

//     use super::*;

//     pub async fn extract_urls(sender: mpsc::Sender<Site>) -> Result<(), ()> {
//         let (trans, recv) = mpsc::unbounded_channel();

//         let _ = try_join!(page_spawner(trans), get_detail_url(sender, recv));

//         Ok(())
//     }

//     async fn page_spawner(sender: mpsc::UnboundedSender<String>) -> Result<(), ()> {
//         let base_url = "https://schoolscompass.com.ng/schools/";

//         // let cloned_sender = sender.clone();
//         // task::spawn(async move {
//         //     extract_pages(cloned_sender, "primary", &base_url).await;
//         // });

//         task::spawn(async move {
//             extract_pages(sender, "secondary", &base_url).await;
//         });

//         Ok(())
//     }

//     async fn get_detail_url(
//         sender: mpsc::Sender<Site>,
//         mut url_reciever: mpsc::UnboundedReceiver<String>,
//     ) -> Result<(), ()> {
//         let mut hanlers = Vec::new();

//         while let Some(url) = url_reciever.recv().await {
//             let sender_clone = sender.clone();
//             let handler = task::spawn(async move {
//                 if let Ok(res) = reqwest::get(url).await {
//                     if let Ok(html) = res.text().await {
//                         let urls = {
//                             let document = Html::parse_document(&html);

//                             let scroller_selector =
//                                 Selector::parse(r#".para-1.btn-school-detail"#).unwrap();

//                             document
//                                 .select(&scroller_selector)
//                                 .filter_map(|li| li.attr("url"))
//                                 .map(|url| url.to_string())
//                                 .collect::<Vec<_>>()
//                         };

//                         print!("============{}", urls.len());

//                         for url in urls {
//                             sender_clone
//                                 .send(Site {
//                                     url: url.to_string(),
//                                     handler: Handlers::SchoolCompass,
//                                     should_sleep: true,
//                                 })
//                                 .await
//                                 .unwrap_or(());
//                         }
//                     }
//                 }
//             });

//             hanlers.push(handler);
//         }

//         println!("--{}--", hanlers.len());

//         for handler in hanlers {
//             let _ = handler.await;
//         }

//         Ok(())
//     }

//     async fn extract_pages(
//         sender: mpsc::UnboundedSender<String>,
//         school_type: &str,
//         base_url: &str,
//     ) {
//         let url = format!("{}{}", base_url, school_type);
//         println!("==============={url}");
//         if let Ok(data) = reqwest::get(url.clone()).await {
//             println!("====================ok");
//             if let Ok(html) = data.text().await {
//                 let pages = {
//                     let document = Html::parse_document(&html);

//                     let scroller_selector = Selector::parse(r#"a[class="page-link"]"#).unwrap();
//                     document
//                         .select(&scroller_selector)
//                         .flat_map(|v| v.text())
//                         .filter_map(|text| text.trim().parse::<u32>().ok())
//                         .last()
//                         .expect("did not get a number")
//                 };

//                 println!("pages{pages}");

//                 for n in 1..(pages + 1) {
//                     let _ = sender.send(format!("{}/?page={}", url, n));
//                 }
//             }
//         }
//     }
// }
