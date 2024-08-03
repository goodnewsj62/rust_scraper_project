use serde::{Deserialize,  Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: String,
    pub message: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub total: String,
    pub current_page: String,
    pub total_pages: i64,
    pub schools: Vec<School>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
    pub id: String,
    pub name: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub address: String,
    pub country: String,
    pub state: String,
    pub city: String,
    pub discount: Option<String>,
    pub slug: String,
    pub logo: String,
    pub rating: String,
    pub lga: Option<String>,
    pub banner: String,
    pub owner: String,
    pub views: String,
    pub likes: String,
    pub is_verified: bool,
    pub is_featured: bool,
    pub is_active: bool,
    pub created_by: String
}