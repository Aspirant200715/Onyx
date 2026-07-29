use crate::{
    headers::Header,
    status::StatusCode,
};

/// Represents an HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Vec<Header>,
    pub body: String,
}


impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
    self.headers.push(Header {
        name: name.into(),
        value: value.into(),
    });

    self
   }

   pub fn body(mut self, body: impl Into<String>) -> Self {
      self.body = body.into();
      self
   }
 

  pub fn status(mut self, status: StatusCode) -> Self {
    self.status = status;
    self
   }


   pub fn serialize(&self) -> Vec<u8> {
    let mut response = String::new();
    // Status line
    response.push_str(&format!(
        "HTTP/1.1 {} {}\r\n",
        self.status.as_u16(),
        self.status.reason_phrase(),
    ));
    // Headers
    for header in &self.headers {
        response.push_str(&format!(
            "{}: {}\r\n",
            header.name,
            header.value,
        ));
    }
    // Content-Length
    response.push_str(&format!(
        "Content-Length: {}\r\n",
        self.body.len(),
    ));
    // Blank line
    response.push_str("\r\n");
    // Body
    response.push_str(&self.body);
    response.into_bytes()
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::StatusCode;

    #[test]
    fn creates_empty_response() {
        let response = Response::new(StatusCode::Ok);

        assert_eq!(response.status, StatusCode::Ok);
        assert!(response.headers.is_empty());
        assert!(response.body.is_empty());
    }

    #[test]
    fn response_is_mutable() {
        let mut response = Response::new(StatusCode::Ok);

        response.body = "Hello Ember".into();

        assert_eq!(response.body, "Hello Ember");
    }
}

#[test]
fn builds_response() {
    let response = Response::new(StatusCode::Ok)
        .header("Content-Type", "text/plain")
        .header("Server", "Ember")
        .body("Hello");

    assert_eq!(response.status, StatusCode::Ok);

    assert_eq!(response.headers.len(), 2);

    assert_eq!(response.body, "Hello");

    assert_eq!(response.headers[0].name, "Content-Type");
}


#[test]
fn serializes_response() {
    let bytes = Response::new(StatusCode::Ok)
        .header("Content-Type", "text/plain")
        .body("Hello")
        .serialize();

    let response = String::from_utf8(bytes).unwrap();

    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Type: text/plain"));
    assert!(response.contains("Content-Length: 5"));
    assert!(response.ends_with("Hello"));
}