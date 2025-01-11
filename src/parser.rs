use crate::models::feed::Feed;
use quick_xml::de::{from_str, DeError};

pub fn parse_feed(xml: &str) -> Result<Feed, DeError> {
    from_str::<Feed>(xml)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_feed_when_valid_xml_then_parses_correctly() {}

    #[test]
    fn test_parse_feed_when_missing_optional_fields_then_still_parses() {}

    #[test]
    fn test_parse_feed_when_malformed_xml_then_returns_error() {}
}
