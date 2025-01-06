use arxiv_wrapper::client::ArxivClient;

#[tokio::main]
async fn main() {
    let query = "search_query=all:electron&start=0&max_results=10";
    let arxiv_client = ArxivClient::new();

    match arxiv_client.query(query).await {
        Ok(body) => println!("body = {body:?}"),
        Err(e) => eprintln!("Error fetching body: {e}"),
    }
}
