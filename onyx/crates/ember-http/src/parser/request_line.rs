use crate::{error::ParserError, method::Method, version::HttpVersion};

pub fn parse_request_line(request: &str) -> Result<&str, ParserError> {
    request
        .lines()
        .next()
        .ok_or(ParserError::MissingRequestLine)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_request_line() {
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let line = parse_request_line(request);
        assert_eq!(line, Ok("GET / HTTP/1.1"));
    }

    #[test]
    fn empty_request_returns_error() {
        let line = parse_request_line("");
        assert_eq!(line, Err(ParserError::MissingRequestLine));
    }

    #[test]
    fn parses_get_method() {
        let method = parse_method("GET / HTTP/1.1");
        assert_eq!(method, Ok(Method::Get));
    }

    #[test]
    fn parses_post_method() {
        let method = parse_method("POST /users HTTP/1.1");
        assert_eq!(method, Ok(Method::Post));
    }

    #[test]
    fn invalid_method_returns_error() {
        let method = parse_method("HELLO / HTTP/1.1");
        assert_eq!(method, Err(ParserError::InvalidMethod));
    }

    #[test]
    fn parses_http_11() {
        let version = parse_version("GET / HTTP/1.1");
        assert_eq!(version, Ok(HttpVersion::Http11));
    }

    #[test]
    fn parses_http_10() {
        let version = parse_version("GET / HTTP/1.0");
        assert_eq!(version, Ok(HttpVersion::Http10));
    }

    #[test]
    fn parses_http_2() {
        let version = parse_version("GET / HTTP/2");
        assert_eq!(version, Ok(HttpVersion::Http2));
    }

    #[test]
    fn invalid_version_returns_error() {
        let version = parse_version("GET / FTP/1.0");
        assert_eq!(version, Err(ParserError::InvalidVersion));
    }
}
