use crate::extractor::FromRequest;
use ember_http::request::Request;

/// Path extractor wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path<T>(pub T);

impl FromRequest for Path<String> {
    type Error = &'static str;

    fn from_request(_: &Request) -> Result<Self, Self::Error> {
        Err("Path extractor not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_path_wrapper() {
        let path = Path(String::from("42"));

        assert_eq!(path.0, "42");
    }
}