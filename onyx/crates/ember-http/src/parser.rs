
use crate::
{method::Method,
version::HttpVersion,
request::Request,
headers::Header,
};
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


    pub fn parse_version(line: &str) -> Option<HttpVersion> {
    let version = line.split_whitespace().nth(2)?;

    match version {
        "HTTP/1.0" => Some(HttpVersion::Http10),
        "HTTP/1.1" => Some(HttpVersion::Http11),
        "HTTP/2.0" | "HTTP/2" => Some(HttpVersion::Http2),
        _ => None,
    }
  }



  pub fn parse_path(line: &str) -> Option<String> {
    line
        .split_whitespace()
        .nth(1)
        .map(String::from)
  }
  

  pub fn parse_request(request: &str) -> Option<Request> {
    let line = Self::request_line(request)?;

    let method = Self::parse_method(line)?;
    let path = Self::parse_path(line)?;
    let version = Self::parse_version(line)?;

    Some(Request {
        method,
        path,
        version,
        headers: Self::parse_headers(request),
    })
  }

  pub fn parse_header(line: &str) -> Option<Header> {
    let (name, value) = line.split_once(':')?;

    Some(Header {
        name: name.trim().to_string(),
        value: value.trim().to_string(),
    })
 }
 
 pub fn parse_headers(request: &str) -> Vec<Header> {
    request
        .lines()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .filter_map(Self::parse_header)
        .collect()
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


#[test]
fn parses_http_11() {
    let version =
        HttpParser::parse_version("GET / HTTP/1.1");

    assert_eq!(version, Some(HttpVersion::Http11));
}

#[test]
fn parses_http_10() {
    let version =
        HttpParser::parse_version("GET / HTTP/1.0");

    assert_eq!(version, Some(HttpVersion::Http10));
}

#[test]
fn parses_http_2() {
    let version =
        HttpParser::parse_version("GET / HTTP/2");

    assert_eq!(version, Some(HttpVersion::Http2));
}

#[test]
fn invalid_version_returns_none() {
    let version =
        HttpParser::parse_version("GET / FTP/1.0");

    assert_eq!(version, None);
}

#[test]
fn parses_complete_request() {
    let raw =
        "GET /users HTTP/1.1\r\nHost: localhost\r\n\r\n";

    let request =
        HttpParser::parse_request(raw).unwrap();

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

    let headers = HttpParser::parse_headers(raw);

    assert_eq!(headers.len(), 2);

    assert_eq!(headers[0].name, "Host");
    assert_eq!(headers[0].value, "localhost");

    assert_eq!(headers[1].name, "User-Agent");
    assert_eq!(headers[1].value, "Ember");
}