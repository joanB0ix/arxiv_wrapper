use crate::{
    error::ArxivError,
    models::Query,
    parser::{parse_feed, Feed},
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

        let xml = response.text().await?; // the big XML string

        let feed = parse_feed(&xml)?;
        Ok(feed)
    }
}
