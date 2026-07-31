use crate::extractor::FromRequest;
use ember_http::request::Request;

/// Path extractor wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path<T>(pub T);

impl FromRequest for Path<String> {
    type Error = &'static str;

    fn from_request(request: &Request) -> Result<Self, Self::Error> {
        if let Some((_, value)) = request.params.iter().next() {
            Ok(Path(value.clone()))
        } else {
            Err("No path parameter found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use ember_http::{
        headers::Header,
        method::Method,
        version::HttpVersion,
    };

    #[test]
    fn creates_path_wrapper() {
        let path = Path(String::from("42"));

        assert_eq!(path.0, "42");
    }

    #[test]
    fn extracts_first_parameter() {
        let mut params = HashMap::new();

        params.insert("id".into(), "42".into());

        let request = Request {
            method: Method::Get,
            path: "/users/42".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params,
        };

        let path = Path::<String>::from_request(&request).unwrap();

        assert_eq!(path.0, "42");
    }

    #[test]
    fn returns_error_when_no_parameter_exists() {
        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: HashMap::new(),
        };

        assert!(Path::<String>::from_request(&request).is_err());
    }
}