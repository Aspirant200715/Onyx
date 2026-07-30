use crate::{error::ParserError, request::Request};
use std::collections::HashMap;

use super::{
    headers::parse_headers,
    request_line::{parse_method, parse_path, parse_request_line, parse_version},
};

/// HTTP parser entry point.
pub struct HttpParser;

impl HttpParser {
    pub fn parse_request(request: &str) -> Result<Request, ParserError> {
        let line = parse_request_line(request)?;

        Ok(Request {
            method: parse_method(line)?,
            path: parse_path(line)?,
            version: parse_version(line)?,
            headers: parse_headers(request)?,
            params: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{method::Method, version::HttpVersion};

    #[test]
    fn parses_complete_request() {
        let raw = "GET /users HTTP/1.1\r\nHost: localhost\r\n\r\n";

        let request = HttpParser::parse_request(raw).unwrap();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/users");
        assert_eq!(request.version, HttpVersion::Http11);
        assert_eq!(request.headers.len(), 1);
        assert_eq!(request.headers[0].name, "Host");
    }
}
