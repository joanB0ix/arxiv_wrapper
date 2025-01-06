use reqwest::Error;

pub struct ArxivClient {
    base_url: String,
}

impl ArxivClient {
    pub fn new() -> Self {
        ArxivClient {
            base_url: "http://export.arxiv.org/api/query".to_string(),
        }
    }

    pub async fn query(&self, query: &str) -> Result<String, Error> {
        reqwest::get(format!("{}?{}", self.base_url, query))
            .await?
            .text()
            .await
    }
}
