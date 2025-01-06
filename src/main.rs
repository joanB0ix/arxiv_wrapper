use arxiv_wrapper::{client::ArxivClient, models::Query};

#[tokio::main]
async fn main() {
    let query = Query::new("all:electron".to_string(), 0, 10);
    let arxiv_client = ArxivClient::new();

    match arxiv_client.query(query).await {
        Ok(body) => println!("body = {body:?}"),
        Err(e) => eprintln!("Error fetching body: {e}"),
    }
}
