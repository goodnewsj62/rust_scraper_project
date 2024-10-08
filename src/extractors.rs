#[derive(Debug)]
pub enum Handlers {
    Edusko,
    Ghanayello,
    SchoolCompass,
    GoAfricaOnline,
}

impl Handlers {
    pub fn handle() {}
}

// struct SchoolInfo {
//     name: String,
//     hash_name: String,
//     address: String,
//     kind: String,
//     from: String,
//     logo: Option<String>,
//     description: Option<String>,
//     is_enabled: bool,
//     date_created_utc: String,
//     date_updated_utc: String,
// }

// pub struct Extract {
//     data: String,
//     bs64_logo: Option<String>, //use byte instead
// }

/// handler strategy that can extract data synchronously
pub mod handlers {
    use std::collections::HashMap;

    use crate::initializers::schema::Root;
    use regex::Regex;
    use scraper::{selectable::Selectable, ElementRef, Html, Selector};

    pub fn edusko_data_extractor(data: &str) -> Vec<HashMap<&'static str, String>> {
        // PARSE JSON
        // loop through data
        // save db

        let reg = Regex::new("\"currentPage\".+").unwrap();
        println!("{:#?}", reg.find(data));

        let parsed_value: Root = serde_json::from_str(data).unwrap();

        parsed_value
            .data
            .schools
            .into_iter()
            .filter(|school| {
                school.category.eq("nursery_and_primary") || school.category.eq("high_school")
            })
            .map(|school| {
                let level = if school.category.eq("high_school") {
                    String::from("secondary")
                } else {
                    String::from("nursery & primary")
                };

                let mut res = HashMap::from_iter([
                    ("name", school.name),
                    ("level", level),
                    (
                        "location",
                        format!("{} {} {}", school.address, school.city, school.state),
                    ),
                    ("country", school.country),
                    ("city", school.state),
                ]);

                if let Some(logo) = school.logo {
                    res.insert("logo", logo);
                }

                res
            })
            .collect()
    }

    pub fn ghanayello_data_extractor(data: &str) -> HashMap<&'static str, String> {
        //  PARSE Content
        //  Extract data using scrapper
        // save  db

        let mut map = HashMap::new();

        let document = Html::parse_document(data);
        let selector = Selector::parse(".info").expect("ganayello info selector");

        for info in document.select(&selector) {
            let label = Selector::parse(".label").expect("ganayello label selector");
            let text = Selector::parse(".text").expect("ganayello text selector");
            let extract = (info.select(&label).next(), info.select(&text).next());

            if let (Some(label), Some(text)) = extract {
                let label = label.text().collect::<String>().trim().to_lowercase();

                let text = text.text().collect::<String>();

                match label.as_str() {
                    "school name" => {
                        map.insert("name", text);
                    }
                    "location" => {
                        // map.insert("a", v);
                        let reg = Regex::new("[vV]iew [mM]ap.*").unwrap();
                        let text = reg.replace(&text, "");
                        map.insert("location", text.trim().to_string());
                    }
                    "contact number" => {
                        map.insert("phone", text);
                    }
                    _ => (),
                }
            }
        }

        map.insert("level", String::from("secondary & primary"));
        map.insert("country", String::from("ghana"));

        map
    }

    pub fn goafrica_data_extractor(data: &str) -> Vec<HashMap<&'static str, String>> {
        let document = Html::parse_document(data);
        let selector =
            Selector::parse("article").expect("goafrica extractor first selector failed");

        document
            .select(&selector)
            .map(|article| {
                let mut map = HashMap::new();

                let header_slector =
                    Selector::parse("h2").expect("goafrica extractor header selector failed");
                let address_selector =
                    Selector::parse("address").expect("goafrica extractor address selector failed");

                let extracts = (
                    article.select(&header_slector).next(),
                    article.select(&address_selector).next(),
                );

                if let (Some(header), Some(address)) = extracts {
                    let name = header.text().collect::<String>().trim().to_string();
                    let location = address.text().collect::<String>().trim().to_string();

                    let school_type = header
                        .parent()
                        .unwrap()
                        .children()
                        .filter_map(ElementRef::wrap)
                        .last();

                    let phone = address
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .children()
                        .filter_map(ElementRef::wrap)
                        .last();
                    let other_info = (school_type, phone);

                    if let (Some(sch_type), Some(phone)) = other_info {
                        if sch_type
                            .text()
                            .collect::<String>()
                            .trim()
                            .to_lowercase()
                            .eq("ecoles secondaires")
                        {
                            map.insert("level", "secondary".to_string());
                        } else {
                            map.insert("level", "primary".to_string());
                        }

                        let reg = Regex::new("(Gsm:|Tel:)").unwrap();
                        let phone = reg
                            .replace(phone.text().collect::<String>().trim(), "")
                            .trim()
                            .to_string();

                        let name = Regex::new(r"\s+").unwrap().replace(&name, " ").to_string();

                        map.insert("name", name);
                        map.insert("location", location);
                        map.insert("phone", phone);
                        map.insert("country", String::from("cameroon"));

                        println!("{map:?}");
                    } else {
                        //LOG
                        println!("goafrica extractor header do not have a next sibling anymore");
                    }
                }

                map
            })
            .collect()
    }

    pub fn school_compass_extractor(data: &str) -> HashMap<&'static str, String> {
        let document = Html::parse_document(data);
        let name_selector =
            Selector::parse(".detail-page-custom-main-div .paraBox ul li:first-child")
                .expect("invalid selector");
        let location_select =
            Selector::parse("#contact-address-block li:last-child").expect("invalid selector");
        let level_selector = Selector::parse("#educational-scope-block").expect("invalid selector");
        let uls_selector =
            Selector::parse(".list-unstyled.margin-bottom-zero").expect("Invalid selector");

        let name = document
            .select(&name_selector)
            .next()
            .expect("oops! could not extract name")
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let location = document
            .select(&location_select)
            .next()
            .expect("oops! could not extract location")
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let phone_section_text = document
            .select(&uls_selector)
            .take(8)
            .last()
            .expect("no 8 element")
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let level_section_text = document
            .select(&level_selector)
            .next()
            .expect("oops! could not extract level")
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let is_primary = Regex::new("[Pp]rimary")
            .unwrap()
            .is_match(&level_section_text);

        let mut store = HashMap::from_iter([
            ("name", name),
            ("location", location),
            ("country", "nigeria".to_owned()),
            (
                "level",
                if is_primary {
                    "nursery & primary".to_owned()
                } else {
                    "secondary".to_owned()
                },
            ),
        ]);

        if let Some(matched) = Regex::new(r#"([0-9].+\s[0-9].+\s[0-9].+)"#)
            .unwrap()
            .captures(&phone_section_text)
        {
            store.insert("phone", String::from(&matched[0]));
        }

        store
    }
}
