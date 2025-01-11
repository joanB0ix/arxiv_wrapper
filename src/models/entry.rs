use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "updated")]
    pub updated: Option<String>,

    #[serde(rename = "published")]
    pub published: Option<String>,

    #[serde(rename = "summary")]
    pub summary: Option<String>,

    #[serde(rename = "author")]
    pub authors: Option<Vec<Author>>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    #[serde(rename = "name")]
    pub name: Option<String>,
}
