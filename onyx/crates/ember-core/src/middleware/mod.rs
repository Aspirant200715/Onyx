pub mod logger;
use ember_http::{request::Request, response::Response};
pub use logger::Logger;
use std::sync::Arc;

impl Next {
    pub fn from_handler<F>(handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        Self::new(handler)
    }
}

/// Trait implemented by all Ember middleware.
pub trait Middleware: Send + Sync {
    fn handle(&self, request: Request, next: &Next) -> Response;
}

pub struct Next {
    handler: Arc<dyn Fn(Request) -> Response + Send + Sync>,
}

impl Next {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        Self {
            handler: Arc::new(handler),
        }
    }

    pub fn run(&self, request: Request) -> Response {
        (self.handler)(request)
    }
}

impl Clone for Next {
    fn clone(&self) -> Self {
        Self {
            handler: Arc::clone(&self.handler),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ember_http::{
        headers::Header, method::Method, request::Request, response::Response, status::StatusCode,
        version::HttpVersion,
    };

    struct DummyMiddleware;

    impl Middleware for DummyMiddleware {
        fn handle(&self, request: Request, next: &Next) -> Response {
            next.run(request)
        }
    }

    #[test]
    fn next_calls_handler() {
        let next = Next::new(|_| Response::new(StatusCode::Ok).body("OK"));

        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: Default::default(),
            query: Default::default(),
        };

        let response = next.run(request);

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.body, "OK");
    }

    #[test]
    fn middleware_calls_next() {
        let middleware = DummyMiddleware;
        let next = Next::new(|_| Response::new(StatusCode::Ok).body("OK"));

        let request = Request {
            method: Method::Get,
            path: "/".into(),
            version: HttpVersion::Http11,
            headers: Vec::<Header>::new(),
            params: Default::default(),
            query: Default::default(),
        };

        let response = middleware.handle(request, &next);

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.body, "OK");
    }
}
