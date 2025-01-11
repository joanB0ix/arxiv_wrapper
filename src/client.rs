use crate::{
    error::ArxivError,
    models::{feed::Feed, query::Query},
    parser::parse_feed,
};
use reqwest::Client;

const ARXIV_API_BASE_URL: &str = "http://export.arxiv.org/api/query";

pub struct ArxivClient {
    http_client: Client,
    base_url: &'static str,
}

impl ArxivClient {
    pub fn new() -> Self {
        ArxivClient {
            http_client: Client::new(),
            base_url: ARXIV_API_BASE_URL,
        }
    }

    pub async fn query(&self, query: Query) -> Result<Feed, ArxivError> {
        let url = format!("{}?{}", self.base_url, query.to_query_string());
        let response = self.http_client.get(&url).send().await?;

        let xml = response.text().await?;

        let feed = parse_feed(&xml)?;

        Ok(feed)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_query_when_valid_feed_then_returns_parsed_feed() {}

    #[tokio::test]
    async fn test_query_when_non_200_status_then_returns_error() {}

    #[tokio::test]
    async fn test_query_when_malformed_xml_then_returns_parse_error() {}
}
