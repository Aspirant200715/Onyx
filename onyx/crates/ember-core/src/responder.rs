use crate::error::EmberError;
/// Represents a type that can become an HTTP response.
use ember_http::{response::Response, status::StatusCode};

impl Responder for EmberError {
    fn into_response(self) -> Response {
        Response::new(self.status_code())
            .header("Content-Type", "text/plain")
            .body(self.to_string())
    }
}

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
    let response = Response::new(StatusCode::Ok).body("Hello");

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

#[test]
fn ember_error_is_responder() {
    let response = EmberError::NotFound.into_response();

    assert_eq!(response.status, StatusCode::NotFound);
    assert_eq!(response.headers[0].value, "text/plain");
    assert_eq!(response.body, "404 Not Found");
}

#[test]
fn bad_request_error_is_responder() {
    let response = EmberError::BadRequest.into_response();

    assert_eq!(response.status, StatusCode::BadRequest);
}

impl<T> Responder for Result<T, EmberError>
where
    T: Responder,
{
    fn into_response(self) -> Response {
        match self {
            Ok(value) => value.into_response(),
            Err(error) => error.into_response(),
        }
    }
}

#[test]
fn ok_result_is_responder() {
    let response: Response = Ok::<_, EmberError>("Hello").into_response();

    assert_eq!(response.status, StatusCode::Ok);
    assert_eq!(response.body, "Hello");
}

#[test]
fn err_result_is_responder() {
    let response: Response = Err::<&'static str, EmberError>(EmberError::NotFound).into_response();

    assert_eq!(response.status, StatusCode::NotFound);
}
