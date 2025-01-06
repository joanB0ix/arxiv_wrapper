use reqwest::Error;

#[tokio::main]
async fn main() {
    match get_body().await {
        Ok(body) => println!("body = {body:?}"),
        Err(e) => eprintln!("Error fetching body: {e}"),
    }
}

async fn get_body() -> Result<String, Error> {
    reqwest::get(
        "http://export.arxiv.org/api/query?search_query=all:electron&start=0&max_results=10",
    )
    .await?
    .text()
    .await
}
