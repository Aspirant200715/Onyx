/// Built-in request logger middleware.
pub struct Logger;

use std::time::Instant;

use ember_http::{request::Request, response::Response};

use super::{Middleware, Next};

impl Middleware for Logger {
    fn handle(&self, request: Request, next: &Next) -> Response {
        println!("[INFO] --> {:?} {}", request.method, request.path);

        let start = Instant::now();

        let response = next.run(request);

        let elapsed = start.elapsed();

        println!(
            "[INFO] <-- {} {} ({:.2?})",
            response.status.as_u16(),
            response.status.reason_phrase(),
            elapsed,
        );

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ember_http::{
        headers::Header, method::Method, request::Request, response::Response, status::StatusCode,
        version::HttpVersion,
    };

    #[test]
    fn logger_passes_request() {
        let logger = Logger;

        let next = Next::new(|_| Response::new(StatusCode::Ok).body("OK"));

        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: Default::default(),
            query: Default::default(),
        };

        let response = logger.handle(request, &next);

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.body, "OK");
    }
}
