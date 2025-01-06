pub struct Query {
    search_query: String,
    start: usize,
    max_results: usize,
}

impl Query {
    pub fn new(search_query: String, start: usize, max_results: usize) -> Self {
        Query {
            search_query,
            start,
            max_results,
        }
    }

    pub fn to_query_string(&self) -> String {
        format!(
            "search_query={}&start={}&max_results={}",
            self.search_query, self.start, self.max_results
        )
    }
}
