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
        let full_path = parse_path(line)?;
        let (path, query) = parse_query_string(full_path);

        Ok(Request {
            method: parse_method(line)?,
            path,
            query,
            version: parse_version(line)?,
            headers: parse_headers(request)?,
            params: HashMap::new(),
        })
    }
}

fn parse_query_string(path: String) -> (String, HashMap<String, String>) {
    let mut parts = path.splitn(2, '?');
    let clean_path = parts.next().unwrap().to_string();
    let mut query = HashMap::new();

    if let Some(query_string) = parts.next() {
        for pair in query_string.split('&') {
            let mut kv = pair.splitn(2, '=');
            let key = kv.next().unwrap_or("");
            let value = kv.next().unwrap_or("");

            if !key.is_empty() {
                query.insert(key.to_string(), value.to_string());
            }
        }
    }

    (clean_path, query)
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

    #[test]
    fn parses_query_string_test() {
        let (path, query) = parse_query_string(
            "/search?q=rust&page=2".to_string(),
        );

        assert_eq!(path, "/search");
        assert_eq!(query.get("q"), Some(&"rust".to_string()));
        assert_eq!(query.get("page"), Some(&"2".to_string()));
   }
}
