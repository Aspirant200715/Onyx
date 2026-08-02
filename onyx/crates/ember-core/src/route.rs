use ember_http::{method::Method, request::Request, response::Response};

use std::sync::Arc;

pub type Handler = Arc<dyn Fn(Request) -> Response + Send + Sync>;

#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Handler,
}

impl Route {
    pub fn new(method: Method, path: impl Into<String>, handler: Handler) -> Self {
        Self {
            method,
            path: path.into(),
            handler,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ember_http::{response::Response, status::StatusCode};

    fn home(_: Request) -> Response {
        Response::new(StatusCode::Ok).body("Home")
    }

    #[test]
    fn creates_route() {
        let route = Route::new(Method::Get, "/", Arc::new(home));

        assert_eq!(route.method, Method::Get);
        assert_eq!(route.path, "/");
    }
}
