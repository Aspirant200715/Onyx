
use crate::method::Method;
pub struct HttpParser;



impl HttpParser {
    pub fn request_line(request: &str) -> Option<&str> {
        request.lines().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_request_line() {
        let request =
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";

        let line = HttpParser::request_line(request);

        assert_eq!(line, Some("GET / HTTP/1.1"));
    }

    #[test]
    fn empty_request_returns_none() {
        let line = HttpParser::request_line("");

        assert_eq!(line, None);
    }
}


impl HttpParser {
    pub fn parse_method(line: &str) -> Option<Method> {
        let method = line.split_whitespace().next()?;

        match method {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "PUT" => Some(Method::Put),
            "DELETE" => Some(Method::Delete),
            "PATCH" => Some(Method::Patch),
            "HEAD" => Some(Method::Head),
            "OPTIONS" => Some(Method::Options),
            _ => None,
        }
    }
}


#[test]
fn parses_get_method() {
    let method = HttpParser::parse_method("GET / HTTP/1.1");

    assert_eq!(method, Some(Method::Get));
}

#[test]
fn parses_post_method() {
    let method = HttpParser::parse_method("POST /users HTTP/1.1");

    assert_eq!(method, Some(Method::Post));
}

#[test]
fn invalid_method_returns_none() {
    let method = HttpParser::parse_method("HELLO / HTTP/1.1");

    assert_eq!(method, None);
}