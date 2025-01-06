use crate::error::ArxivError;
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

    pub async fn query(&self, query: &str) -> Result<String, ArxivError> {
        let url = format!("{}?{}", self.base_url, query);
        let response = self.http_client.get(&url).send().await?;

        response.text().await.map_err(ArxivError::from)
    }
}
