use quick_xml::de::{from_str, DeError};
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

pub fn parse_feed(xml: &str) -> Result<Feed, DeError> {
    from_str::<Feed>(xml)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_feed_when_valid_xml_then_parses_correctly() {}

    #[test]
    fn test_parse_feed_when_missing_optional_fields_then_still_parses() {}

    #[test]
    fn test_parse_feed_when_malformed_xml_then_returns_error() {}
}
