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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_query_string_happy_path() {
        let q = Query::new("electron".to_string(), 10, 25);
        assert_eq!(
            q.to_query_string(),
            "search_query=electron&start=10&max_results=25"
        )
    }
}
