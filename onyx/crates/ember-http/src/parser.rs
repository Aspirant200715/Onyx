use crate::{
    error::ParserError, headers::Header, method::Method, request::Request, version::HttpVersion,
};
pub struct HttpParser;

impl HttpParser {
    pub fn request_line(request: &str) -> Result<&str, ParserError> {
        request
            .lines()
            .next()
            .ok_or(ParserError::MissingRequestLine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_request_line() {
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";

        let line = HttpParser::request_line(request);

        assert_eq!(line, Ok("GET / HTTP/1.1"));
    }

    #[test]
    fn empty_request_returns_error() {
        let line = HttpParser::request_line("");

        assert_eq!(line, Err(ParserError::MissingRequestLine));
    }
}

impl HttpParser {
    pub fn parse_method(line: &str) -> Result<Method, ParserError> {
        let method = line
            .split_whitespace()
            .next()
            .ok_or(ParserError::InvalidMethod)?;

        match method {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "PATCH" => Ok(Method::Patch),
            "HEAD" => Ok(Method::Head),
            "OPTIONS" => Ok(Method::Options),
            _ => Err(ParserError::InvalidMethod),
        }
    }

    pub fn parse_version(line: &str) -> Result<HttpVersion, ParserError> {
        let version = line
            .split_whitespace()
            .nth(2)
            .ok_or(ParserError::InvalidVersion)?;

        match version {
            "HTTP/1.0" => Ok(HttpVersion::Http10),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            "HTTP/2.0" | "HTTP/2" => Ok(HttpVersion::Http2),
            _ => Err(ParserError::InvalidVersion),
        }
    }

    pub fn parse_path(line: &str) -> Result<String, ParserError> {
        line.split_whitespace()
            .nth(1)
            .map(String::from)
            .ok_or(ParserError::InvalidPath)
    }

    pub fn parse_request(request: &str) -> Result<Request, ParserError> {
        let line = Self::request_line(request)?;

        Ok(Request {
            method: Self::parse_method(line)?,
            path: Self::parse_path(line)?,
            version: Self::parse_version(line)?,
            headers: Self::parse_headers(request)?,
        })
    }

    pub fn parse_header(line: &str) -> Result<Header, ParserError> {
        let (name, value) = line.split_once(':').ok_or(ParserError::MalformedHeader)?;

        Ok(Header {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
        })
    }

    pub fn parse_headers(request: &str) -> Result<Vec<Header>, ParserError> {
        request
            .lines()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(Self::parse_header)
            .collect()
    }
}

#[test]
fn parses_get_method() {
    let method = HttpParser::parse_method("GET / HTTP/1.1");

    assert_eq!(method, Ok(Method::Get));
}

#[test]
fn parses_post_method() {
    let method = HttpParser::parse_method("POST /users HTTP/1.1");

    assert_eq!(method, Ok(Method::Post));
}

#[test]
fn invalid_method_returns_error() {
    let method = HttpParser::parse_method("HELLO / HTTP/1.1");

    assert_eq!(method, Err(ParserError::InvalidMethod));
}

#[test]
fn parses_http_11() {
    let version = HttpParser::parse_version("GET / HTTP/1.1");

    assert_eq!(version, Ok(HttpVersion::Http11));
}

#[test]
fn parses_http_10() {
    let version = HttpParser::parse_version("GET / HTTP/1.0");

    assert_eq!(version, Ok(HttpVersion::Http10));
}

#[test]
fn parses_http_2() {
    let version = HttpParser::parse_version("GET / HTTP/2");

    assert_eq!(version, Ok(HttpVersion::Http2));
}

#[test]
fn invalid_version_returns_error() {
    let version = HttpParser::parse_version("GET / FTP/1.0");

    assert_eq!(version, Err(ParserError::InvalidVersion));
}

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
fn parses_headers() {
    let raw = "\
GET / HTTP/1.1\r\n\
Host: localhost\r\n\
User-Agent: Ember\r\n\
\r\n";

    let headers = HttpParser::parse_headers(raw).unwrap();

    assert_eq!(headers.len(), 2);

    assert_eq!(headers[0].name, "Host");
    assert_eq!(headers[0].value, "localhost");

    assert_eq!(headers[1].name, "User-Agent");
    assert_eq!(headers[1].value, "Ember");
}
