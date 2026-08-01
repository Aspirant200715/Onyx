use crate::extractor::FromRequest;
use ember_http::request::Request;

/// Query extractor wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query<'a> {
    pub name: &'a str,
}

impl<'a> Query<'a> {
    pub fn named(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> FromRequest for Query<'a> {
    type Output = String;
    type Error = &'static str;

    fn extract(self, request: &Request) -> Result<Self::Output, Self::Error> {
        if let Some(value) = request.query.get(self.name) {
            Ok(value.clone())
        } else {
            Err("No query parameter found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_query_wrapper() {
        let query = Query::named("q");

        assert_eq!(query.name, "q");
    }
}

#[cfg(test)]
mod extractor_tests {
    use super::*;

    use std::collections::HashMap;

    use ember_http::{headers::Header, method::Method, request::Request, version::HttpVersion};

    #[test]
    fn extracts_first_query_parameter() {
        let mut query = HashMap::new();

        query.insert("q".into(), "rust".into());

        let request = Request {
            method: Method::Get,
            path: "/search".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: HashMap::new(),
            query,
        };

        let value = Query::named("q").extract(&request).unwrap();

        assert_eq!(value, "rust");
    }

    #[test]
    fn returns_error_when_query_missing() {
        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: HashMap::new(),
            query: HashMap::new(),
        };

        assert!(Query::named("q").extract(&request).is_err());
    }
}
