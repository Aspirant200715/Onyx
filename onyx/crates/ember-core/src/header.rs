use crate::extractor::FromRequest;
use ember_http::request::Request;

/// Header extractor wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub name: &'a str,
}

impl<'a> Header<'a> {
    pub fn named(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> FromRequest for Header<'a> {
    type Output = String;
    type Error = &'static str;

    fn from_request(self, request: &Request) -> Result<Self::Output, Self::Error> {
        request
            .headers
            .iter()
            .find(|h| h.name == self.name)
            .map(|h| h.value.clone())
            .ok_or("No header found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use ember_http::{
        headers::Header as HttpHeader,
        method::Method,
        request::Request,
        version::HttpVersion,
    };

    #[test]
    fn extracts_first_header() {
        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: vec![
                HttpHeader {
                    name: "Host".into(),
                    value: "localhost".into(),
                }
            ],
            params: HashMap::new(),
            query: HashMap::new(),
        };

        let value = Header::named("Host").from_request(&request).unwrap();

        assert_eq!(value, "localhost");
    }

    #[test]
    fn returns_error_when_no_headers_exist() {
        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::new(),
            params: HashMap::new(),
            query: HashMap::new(),
        };

        assert!(Header::named("Host").from_request(&request).is_err());
    }
}