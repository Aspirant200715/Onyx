/// Represents a type that can become an HTTP response.
use ember_http::{
    response::Response,
    status::StatusCode,
};

pub trait Responder {
    fn into_response(self) -> Response;
}

impl Responder for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl Responder for &'static str {
    fn into_response(self) -> Response {
        Response::new(StatusCode::Ok)
            .header("Content-Type", "text/plain")
            .body(self)
    }
}


impl Responder for String {
    fn into_response(self) -> Response {
        Response::new(StatusCode::Ok)
            .header("Content-Type", "text/plain")
            .body(self)
    }
}

#[test]
fn response_is_responder() {
    let response = Response::new(StatusCode::Ok)
        .body("Hello");

    let converted = response.into_response();

    assert_eq!(converted.status, StatusCode::Ok);
}

#[test]
fn str_is_responder() {
    let response = "Hello".into_response();

    assert_eq!(response.status, StatusCode::Ok);
}

#[test]
fn string_is_responder() {
    let response = "Hello".to_string().into_response();

    assert_eq!(response.status, StatusCode::Ok);
}