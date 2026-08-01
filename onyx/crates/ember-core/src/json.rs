use serde::Serialize;

/// JSON response wrapper.
#[derive(Debug, Clone)]
pub struct Json<T>(pub T);


use crate::responder::Responder;

use ember_http::{
    response::Response,
    status::StatusCode,
};

impl<T> Responder for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self.0)
            .expect("JSON serialization failed");

        Response::new(StatusCode::Ok)
            .header("Content-Type", "application/json")
            .body(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::responder::Responder;

    use serde::Serialize;

    #[derive(Serialize)]
    struct User {
        id: u32,
        name: String,
    }

    #[test]
    fn serializes_json() {
        let response = Json(User {
            id: 1,
            name: "Ember".into(),
        })
        .into_response();

        assert_eq!(
            response.headers[0].value,
            "application/json",
        );

        assert!(response.body.contains("\"id\":1"));
        assert!(response.body.contains("\"name\":\"Ember\""));
    }
}