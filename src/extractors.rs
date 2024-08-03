#[derive(Debug)]
pub enum Handlers {
    Edusko,
    Ghanayello,
    SchoolCompass,
}

impl Handlers {
    pub fn handle() {}
}

struct SchoolInfo {
    name: String,
    hash_name: String,
    address: String,
    kind: String,
    from: String,
    logo: Option<String>,
    description: Option<String>,
    is_enabled: bool,
    date_created_utc: String,
    date_updated_utc: String,
}

pub struct Extract {
    data: String,
    bs64_logo: Option<String>, //use byte instead
}

/// handler strategy that can extract data synchronously
mod handlers {
    fn edusko_data_handler(data: &str) {
        // PARSE JSON
        // loop through data
        // save db
    }

    fn ghanayello_data_handler(data: &str) {

        //  PARSE Content
        //  Extract data using scrapper
        // save  db
    }
}
