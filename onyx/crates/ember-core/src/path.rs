use crate::extractor::FromRequest;
use ember_http::request::Request;

/// Path extractor wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path<'a> {
    pub name: &'a str,
}

impl<'a> Path<'a> {
    pub fn named(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> FromRequest for Path<'a> {
    type Output = String;
    type Error = &'static str;

    fn extract(self, request: &Request) -> Result<Self::Output, Self::Error> {
        if let Some(value) = request.params.get(self.name) {
            Ok(value.clone())
        } else {
            Err("No path parameter found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ember_http::{headers::Header, method::Method, version::HttpVersion};
    use std::collections::HashMap;

    #[test]
    fn creates_path_wrapper() {
        let path = Path::named("id");

        assert_eq!(path.name, "id");
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
            query: HashMap::new(),
        };

        let id = Path::named("id").extract(&request).unwrap();

        assert_eq!(id, "42");
    }

    #[test]
    fn returns_error_when_no_parameter_exists() {
        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: HashMap::new(),
            query: HashMap::new(),
        };

        assert!(Path::named("id").extract(&request).is_err());
    }
}
