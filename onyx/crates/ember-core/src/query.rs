use crate::extractor::FromRequest;
use ember_http::request::Request;

/// Query extractor wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query<T>(pub T);

impl FromRequest for Query<String> {
    type Error = &'static str;

    fn from_request(_request: &Request) -> Result<Self, Self::Error> {
        Err("Query extractor not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_query_wrapper() {
        let query = Query(String::from("rust"));

        assert_eq!(query.0, "rust");
    }
}
