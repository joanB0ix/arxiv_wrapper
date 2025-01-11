use super::entry::Entry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Feed {
    #[serde(rename = "title")]
    pub feed_title: Option<String>,

    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "updated")]
    pub updated: Option<String>,

    #[serde(rename = "entry")]
    pub entries: Option<Vec<Entry>>,
}
