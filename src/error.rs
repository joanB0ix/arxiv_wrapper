use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArxivError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
}
