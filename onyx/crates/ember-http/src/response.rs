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