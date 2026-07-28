use crate::{error::ParserError, headers::Header};

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
        .map(parse_header)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_headers() {
        let raw = "\
GET / HTTP/1.1\r\n\
Host: localhost\r\n\
User-Agent: Ember\r\n\
\r\n";

        let headers = parse_headers(raw).unwrap();

        assert_eq!(headers.len(), 2);

        assert_eq!(headers[0].name, "Host");
        assert_eq!(headers[0].value, "localhost");

        assert_eq!(headers[1].name, "User-Agent");
        assert_eq!(headers[1].value, "Ember");
    }
}
