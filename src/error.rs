use quick_xml::DeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArxivError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("XML parse error: {0}")]
    XmlParse(#[from] DeError),
}
